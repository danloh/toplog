
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
  //# if show edit link
  let editlink = document.getElementById('edit-link');
  if (editlink) { 
    let omg = getCookie("oMg");
    let check =  omg === 'true';                                                 // for tag|item
    editlink.style.display = check ? '' : 'none';
  }
});

function onSearch(selector, ty) {
  let q = document.getElementById(selector);
  if (q && q.value != "") {
    let openUrl = ty === "g" 
      ? 'https://www.google.com/search?q=site:newdin.com/%20'
      : '/me/item/search?q=';
    window.open(openUrl + q.value, "_blank");
  }
}

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

const PerPage = 42; 
let idxPage = 1;
let hasMoreIdx = true;
function loadMoreItems(topic='all', ty='Article') {
  if (!hasMoreIdx) { return; }
  idxPage += 1;
  axios.get(`/more/${topic}/${ty}?page=${idxPage}&perpage=${PerPage}`)
  .then(function(resp) {
    let data = resp.data || "";
    if ( !Boolean(data) ) {
      console.log("No More");
      hasMoreIdx = false;
    }
    document.getElementById('item-list').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}

function toggleTop(slug) {
  let omg = getCookie("oMg");
  if (omg !== 'true') return;
  let tok = getCookie(TOK);
  axios.defaults.headers.common['Authorization'] = tok;
  axios.patch(`/api/items/${slug}`)
  .then(
    res => console.log(res.data)
  );
}

function upVote(slug) {
  let tok = getCookie(TOK);
  let check = Boolean(tok);
  if (!check) {
    window.location.href = "/me";
    return;
  }
  axios.defaults.headers.common['Authorization'] = tok;
  axios.put(`/api/items/${slug}?action=vote`)
  .then(res => {
    let voteNum = res.data;
    //console.log(voteNum)
    let voteEle = document.getElementById("vote-" + slug);
    if (voteEle) { 
      voteEle.innerText = voteNum; 
      let upEle = document.getElementById("up-" + slug);
      if (upEle) { upEle.hidden = true }
    }
  });
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
