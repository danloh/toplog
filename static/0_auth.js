
let RedirectURL = '/';
document.addEventListener('DOMContentLoaded', async function() {
  if (getCookie(TOK)) {
    let authBox = document.getElementById('auth-box');
    if (authBox) { authBox.style.display = 'none'; }
  }
  let query = document.location.search;
  let docRefer = document.referrer;
  RedirectURL = getQueryParam('redirect', query) 
    || docRefer
    || '/';
  let toWhat = getQueryParam('to', query);
  let toNum = toWhat == 'signin' 
    ? 0 
    : toWhat == 'signup' 
      ? 1 
      : toWhat == 'reset' 
        ? 2 
        : toWhat == 'changepsw' 
          ? 3
          : toWhat == 'update' ? 4 : 0;

  // redirect when authed on signin / signup
  if (toNum < 2 && getCookie(TOK)) {
    window.location.href = RedirectURL;
  }

  if (toNum >= 3 && !getCookie(TOK)) return;
  if (toNum == 4) {
    let name = document.getElementById('hide-uname');
    if (!name) return;
    let uname = name.value;
    if (uname != getCookie(IDENT)) return;
    // set user
    let getUser = await fetch(`/api/users/${uname}`);
    if (!getUser.ok) return;
    let userRes = await getUser.json();
    let userInfo = Object.assign({ },userRes.user);
    // console.log(userInfo);
    let ids = ['nickname', 'avatar', 'email', 'location', 'intro'];
    setValsByIDs(ids, 'auth-up-', userInfo);
  } 

  onSwitch(toNum);
  let titl = document.getElementById('auth-form-title');
  if (titl) {
    titl.innerText = toNum < 2
      ? 'Welcome' 
      : toNum == 2 
        ? 'Recover Password'
        : toNum == 3 
          ? 'Change Password'
          : 'Update Profile';
  }
}, false)

async function login() {
  let ids = ['username', 'psw'];
  let loginfo = getValsByIDs(ids, 'auth-');
  let uname = loginfo[ids.indexOf('username')];
  let password = loginfo[ids.indexOf('psw')];
  if (uname.trim().length < 3 || password.length < 8 ) {
    alert('Invalid Input')
  }
  let authData = { 
    uname: uname.trim(), 
    password: Base64encode(password, true)
  };
  let options = {
    method:  'POST', 
    headers: {'Content-Type': 'application/json'}, 
    body: JSON.stringify(authData)
  };
  let authResp = await fetch('/api/signin', options);
  if (!authResp.ok) { alert("Failed.."); return }

  let auth = await authResp.json();
  setAuth(auth);
  window.location.href = RedirectURL;
}

async function signup() {
  let ids = ['newuser', 'newpsw', 'repsw'];
  let loginfo = getValsByIDs(ids, 'auth-');
  let uname = loginfo[ids.indexOf('newuser')];
  let password = loginfo[ids.indexOf('newpsw')];
  let confirm = loginfo[ids.indexOf('repsw')];
  if (!regName.test(uname.trim()) || !regPsw.test(password)) {
    alert('Invalid Input'); 
    return
  }
  if (password !== confirm) { alert('Password Not Match'); return}

  let authData = { 
    uname: uname.trim(), 
    email: '',
    password: Base64encode(password, true),
    confirm: Base64encode(password, true),
    invitee_code,
    agree: true,
  };
  let options = {
    method:  'POST', 
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify(authData)
  };
  let regResp = await fetch('/api/signup', options);
  if (!regResp.ok) { alert("Failed.."); return }
  let reg = await regResp.json();
  setAuth(reg);
  window.location.href = RedirectURL;
}

function reset() { /*TODO*/ }

async function changePsw() {
  if (!getCookie(TOK)) return;
  let ids = ['oldpsw', 'chpsw', 'rechpsw'];
  let info = getValsByIDs(ids, 'auth-');
  let oldPsw = info[ids.indexOf('oldpsw')];
  let newPsw = info[ids.indexOf('chpsw')];
  let confirm = info[ids.indexOf('rechpsw')];
  if (!regPsw.test(oldPsw) || !regPsw.test(newPsw) || newPsw != confirm) {
    alert('Invalid Input'); 
    return
  }
  let uname = getCookie(IDENT);
  let pswData = { 
    old_psw: Base64encode(oldPsw, true),
    new_psw: Base64encode(newPsw, true),
    uname,
  };
  let options = {
    method:  'PUT', 
    headers: {
      'Content-Type': 'application/json', 
      'Authorization': getCookie(TOK),
    },
    body: JSON.stringify(pswData)
  };
  let chResp = await fetch('/api/users/' + uname, options);
  if (!chResp.ok) { alert("Failed.."); return }
  signOut('/auth?to=signin');
}

const regPsw = /^[\w#@~%^$&*-]{8,18}$/;
//const regPsw = /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[#@!~%^$&*-])[a-zA-Z\d#@!~%^$&*-]{8,18}$/;
const regEmail = /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/;
const regName = /^[\w-]{3,16}$/;

let trigger = 0;  // 0- login, 1- signup, 2- reset 3- change psw 4- update
function onSwitch(t=0) {
  trigger = t;
  let signupForm = document.getElementById('sign-up-form');
  let signinForm = document.getElementById('sign-in-form');
  let resetForm = document.getElementById('reset-form');
  let chpswForm = document.getElementById('chpsw-form');
  let upForm = document.getElementById('update-form');
  if (signinForm) { signinForm.style.display = trigger == 0 ? '' : 'none'; }
  if (signupForm) { signupForm.style.display = trigger == 1 ? '' : 'none'; }
  if (resetForm) { resetForm.style.display = trigger == 2 ? '' : 'none'; }
  if (chpswForm) { chpswForm.style.display = trigger == 3 ? '' : 'none'; }
  if (upForm) { upForm.style.display = trigger == 4 ? '' : 'none'; }
}

function setAuth(reg) {
  setCookie(TOK, reg.token, {expires: reg.exp});
  setCookie(IDENT, reg.user.uname, {expires: reg.exp});
  //setCookie(CAN, reg.user.permission, {expires: reg.exp});
  setCookie('oMg', reg.omg, {expires: reg.exp});
}

async function updateUser() {
  if (!getCookie(TOK)) return;
  let ids = ['nickname', 'avatar', 'email', 'location', 'intro'];
  let info = getValsByIDs(ids, 'auth-up-');
  let uname = getCookie(IDENT);
  let upUser = {
    uname,
    avatar: info[ids.indexOf('avatar')],
    email: info[ids.indexOf('email')],
    intro: info[ids.indexOf('intro')],
    location: info[ids.indexOf('location')],
    nickname: info[ids.indexOf('nickname')],
  };
  let options = {
    method:  'POST', 
    headers: {
      'Content-Type': 'application/json', 
      'Authorization': getCookie(TOK),
    },
    body: JSON.stringify(upUser)
  };
  let upResp = await fetch(`/api/users/${uname}`, options);
  if (!upResp.ok) return;

  window.location.href = '/@' + uname;
}

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
