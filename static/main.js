// =================================================================
//## show dropdown
function showMenu() { 
  showDrop("drop-menu"); 
}
function showOption() { 
  showDrop("drop-opt"); 
}
function showDrop(id_name) {
  let show = document.getElementById(id_name);
  if (show) { show.classList.toggle("show");}
}

const TOK = 'NoSeSNekoTr'; // for get cookie token
const IDENT = 'YITnEdIr'  // for get cookie identity
function getCookie(c_name) {
  let c_start = 0, c_end = 0, ck = document.cookie;
  if (ck.length > 0) {
    c_start = ck.indexOf(c_name + "=");
    if (c_start != -1) {
      c_start = c_start + c_name.length + 1;
      c_end = ck.indexOf(";", c_start);
      if (c_end === -1) { c_end = ck.length;}
    }
    let c = ck.substring(c_start,c_end);
    return unescape(c) 
  }
  return ""
}
//## action once window loaded
window.addEventListener('load', function() {
  //# check if authed
  let iden = getCookie(IDENT);
  document.getElementById('login-link').style.display = iden ? 'none' : '';
  document.getElementById('menu-down').style.display = iden ? '' : 'none';
  document.getElementById('profile-link').href = iden ? '/me/p/' + iden : '#';
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
    let openUrl = 'https://www.google.com/search?q=site:newdin.com/%20';
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
    window.location.href = "/me";
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
    window.location.href = "/me";
    return;
  }
  if (!check) return;
  window.location.href = link;
}
