// base64 js: https://github.com/dankogai/js-base64/blob/master/base64.mjs
//
const _hasbtoa = typeof btoa === 'function';
const _hasBuffer = typeof Buffer === 'function';
const _TE = typeof TextEncoder === 'function' ? new TextEncoder() : undefined;
const b64ch = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';
const b64chs = [...b64ch];

const _fromCC = String.fromCharCode.bind(String);

const _mkUriSafe = (src) => src
  .replace(/[+\/]/g, (m0) => m0 == '+' ? '-' : '_')
  .replace(/=+$/m, '');

const btoaPolyfill = (bin) => {
  // console.log('polyfilled');
  let u32, c0, c1, c2, asc = '';
  const pad = bin.length % 3;
  for (let i = 0; i < bin.length;) {
    if ((c0 = bin.charCodeAt(i++)) > 255 ||
      (c1 = bin.charCodeAt(i++)) > 255 ||
      (c2 = bin.charCodeAt(i++)) > 255)
      throw new TypeError('invalid character found');
    u32 = (c0 << 16) | (c1 << 8) | c2;
    asc += b64chs[u32 >> 18 & 63]
      + b64chs[u32 >> 12 & 63]
      + b64chs[u32 >> 6 & 63]
      + b64chs[u32 & 63];
  }
  return pad ? asc.slice(0, pad - 3) + "===".substring(pad) : asc;
};

const _btoa = _hasbtoa 
  ? (bin) => btoa(bin)
  : _hasBuffer 
    ? (bin) => Buffer.from(bin, 'binary').toString('base64')
    : btoaPolyfill;

const _fromUint8Array = _hasBuffer
  ? (u8a) => Buffer.from(u8a).toString('base64')
  : (u8a) => {
    // cf. https://stackoverflow.com/questions/12710001/how-to-convert-uint8-array-to-base64-encoded-string/12713326#12713326
    const maxargs = 0x1000;
    let strs = [];
    for (let i = 0, l = u8a.length; i < l; i += maxargs) {
      strs.push(_fromCC.apply(null, u8a.subarray(i, i + maxargs)));
    }
    return _btoa(strs.join(''));
  };

// converts a Uint8Array to a Base64 string.
const fromUint8Array = (u8a, urlsafe = false) => urlsafe 
  ? _mkUriSafe(_fromUint8Array(u8a)) 
  : _fromUint8Array(u8a);
// This trick is found broken https://github.com/dankogai/js-base64/issues/130
// const utob = (src: string) => unescape(encodeURIComponent(src));
// reverting good old fationed regexp
const cb_utob = (c) => {
  if (c.length < 2) {
    var cc = c.charCodeAt(0);
    return cc < 0x80 
      ? c
      : cc < 0x800 
        ? (_fromCC(0xc0 | (cc >>> 6)) + _fromCC(0x80 | (cc & 0x3f)))
        : (_fromCC(0xe0 | ((cc >>> 12) & 0x0f))
          + _fromCC(0x80 | ((cc >>> 6) & 0x3f))
          + _fromCC(0x80 | (cc & 0x3f)));
  }
  else {
    var cc = 0x10000
      + (c.charCodeAt(0) - 0xD800) * 0x400
      + (c.charCodeAt(1) - 0xDC00);
    return (_fromCC(0xf0 | ((cc >>> 18) & 0x07))
      + _fromCC(0x80 | ((cc >>> 12) & 0x3f))
      + _fromCC(0x80 | ((cc >>> 6) & 0x3f))
      + _fromCC(0x80 | (cc & 0x3f)));
  }
};
const re_utob = /[\uD800-\uDBFF][\uDC00-\uDFFFF]|[^\x00-\x7F]/g;
const utob = (u) => u.replace(re_utob, cb_utob);
const _encode = _hasBuffer
  ? (s) => Buffer.from(s, 'utf8').toString('base64')
  : _TE
    ? (s) => _fromUint8Array(_TE.encode(s))
    : (s) => _btoa(utob(s));

const Base64encode = (src, urlsafe = false) => urlsafe
  ? _mkUriSafe(_encode(src))
  : _encode(src);
