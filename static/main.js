// =================================================================
//## show dropdown
function showMenu(id_name = "drop-menu") {
  let show = document.getElementById(id_name);
  if (show) { show.classList.toggle("show");}
}

// get query param
String.prototype.regexIndexOf = function(regex, startpos) {
  var indexOf = this.substring(startpos || 0).search(regex);
  return (indexOf >= 0) ? (indexOf + (startpos || 0)) : indexOf;
}

function getParam(param, query, startwith, delimit1, delimit2) {
  let start = 0, end = 0;
  // console.log(query);
  if (query.length > start && query.startsWith(startwith)) {
    start = query.search(param + delimit1);
    if (start != -1) {
      start = start + param.length + 1;
      end = query.regexIndexOf(delimit2, start);
      if (end === -1) { end = query.length;}
    }
    let c = query.substring(start, end);
    return c
  }
  return ""
}

function getQueryParam(param, query) {
  return getParam(param, query, startwith='?', delimit1='=', delimit2=/&[\w]+=/)
}
function getCookie(param) {
  return getParam(param, query=document.cookie, startwith='', delimit1='=', delimit2=';')
}

// extract val or set val
function getValsByIDs(ids=[], prefix='') {
  let vals = [];
  for ( let id of ids ) {
    let ele = document.getElementById(prefix + id);
    let val = ele ? ele.value || ele.innerHTML : '';
    vals.push(val);
  }
  return vals;
}
function setValsByIDs(ids=[], prefix='', vals={}) {
  for ( let id of ids ) {
    let ele = document.getElementById(prefix + id);
    let val = vals[id]
    if (ele.value === undefined) {
      ele.innerHTML = val || '';
    } else {
      ele.value = val || '';
    }
  }
}

const TOK = 'NoSeSNekoTr'; // for get cookie token
const IDENT = 'YITnEdIr'  // for get cookie identity
//## action once window loaded
window.addEventListener('DOMContentLoaded', function() {
  //# check if authed
  let iden = getCookie(IDENT);
  let loginLink = document.getElementById('login-link');
  if (loginLink) { 
    loginLink.setAttribute('href', `/@${iden}`);
    loginLink.innerText = 'Profile';
  } 
});
// Close the dropdown menu if the user clicks outside of it
window.onclick = function(event) {
  if (!event.target.matches('.toolbtn')) {
    let dropdowns = document.getElementsByClassName("dropdown-content");
    let i;
    for (i = 0; i < dropdowns.length; i++) {
      let openDropdown = dropdowns[i];
      if (openDropdown.classList.contains('show')) {
        openDropdown.classList.remove('show');
      }
    }
  }
}

function onSearch(selector) {
  let q = document.getElementById(selector);
  if (q && q.value != "") {
    let openUrl = 'https://www.google.com/search?q=site:toplog.cc/%20';
    window.open(openUrl + q.value, "_blank");
  }
}

const PerPage = 42; 
let idxPage = 1;
let hasMoreIdx = true;
function loadMoreItems(topic='all', ty='Article') {
  if (!hasMoreIdx) { return; }
  idxPage += 1;
  fetch(
    `/more/${topic}/${ty}?page=${idxPage}&perpage=${PerPage}`
  ).then(resp => {
    //console.log(resp);
    resp.text().then( r => {
      //console.log(r);
      if ( !Boolean(r) ) {
        console.log("No More");
        hasMoreIdx = false;
      }
      document.getElementById('item-list').innerHTML += r;
    })
  });
  window.scrollTo(0, document.body.scrollHeight);
}

function toggleTop(slug) {
  let omg = getCookie("oMg");
  if (omg !== 'true') return;
  let tok = getCookie(TOK);
  fetch(`/api/items/${slug}`, {
    method: 'PATCH', 
    headers: { 'Authorization': tok },
  }).then(_res => {
    let toggleEle = document.getElementById("t-" + slug);
    if (toggleEle) { toggleEle.hidden = true }
    //console.log(res.data)
  });
}

function upVote(slug) {
  let tok = getCookie(TOK);
  let check = Boolean(tok);
  if (!check) {
    window.location.href = "/auth?to=signin";
    return;
  }
  fetch(`/api/items/${slug}?action=vote`, {
    method: 'PUT', 
    headers: { 'Authorization': tok },
  }).then(res => {
    res.json().then(r => {
      //console.log(r);
      let voteEle = document.getElementById("vote-" + slug);
      if (voteEle) { 
        //let voteNum = Number(voteEle.innerText);
        voteEle.innerText = r; 
        let upEle = document.getElementById("up-" + slug);
        if (upEle) { upEle.hidden = true }
      }
    }) 
  });
}

// md parse in backend
function showFull(slug) {
  let mdSelector = 'md-' + slug;
  let btnSelector = 'btn-' + slug;
  let btn = document.getElementById(btnSelector);
  let full = document.getElementById(mdSelector);
  let ifShowMore = btn.innerText == 'more...' ? true : false
  full.className = ifShowMore ? 'meta-sum' : 'hide-part meta-sum';
  btn.innerText = ifShowMore ? 'less...' : 'more...';
}

function openLink(link, admin=false) {
  let check = admin 
    ? getCookie("oMg") === "true" 
    : Boolean(getCookie(TOK));
  if (!check && !admin) {
    window.location.href = "/auth?to=signin";
    return;
  }
  if (!check) return;
  window.location.href = link;
}


// auth 
// set cookie
function setCookie (key, value, attributes) {
  if (typeof document === 'undefined') return;

  attributes = Object.assign({secure: true, sameSite: 'Lax'}, attributes);

  if (typeof attributes.expires === 'number') {
    attributes.expires = new Date(Date.now() + attributes.expires * 864e5);
  }
  if (attributes.expires) {
    attributes.expires = attributes.expires.toUTCString();
  }

  key = encodeURIComponent(key)
    .replace(/%(2[346B]|5E|60|7C)/g, decodeURIComponent)
    .replace(/[()]/g, escape);
  value = encodeURIComponent(value)
    .replace(/%(2[346BF]|3[AC-F]|40|5[BDE]|60|7[BCD])/g, decodeURIComponent);

  let stringifiedAttributes = '';
  for (let attributeName in attributes) {
    if (!attributes[attributeName]) {
      continue
    }
    stringifiedAttributes += '; ' + attributeName;
    if (attributes[attributeName] === true) {
      continue
    }
    // Considers RFC 6265 section 5.2:
    // ...
    // 3.  If the remaining unparsed-attributes contains a %x3B (";")
    //     character:
    // Consume the characters of the unparsed-attributes up to,
    // not including, the first %x3B (";") character.
    // ...
    stringifiedAttributes += '=' + attributes[attributeName].split(';')[0];
  }

  return (document.cookie = key + '=' + value + stringifiedAttributes)
}

function delCookie(key, attributes={}) {
  setCookie(key, '', Object.assign({expires: -1}, attributes))
}

function signOut(to='/') {
  delCookie(TOK);
  delCookie(IDENT);
  // delCookie(CAN);
  delCookie('oMg');
  window.location.href = to;
}
