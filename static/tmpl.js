// config marked
const renderer = new marked.Renderer();
function paragraphParse(text) {
  return `<p>\n${text}</p>`;
}
function linkParse(href, title, text) {
  const isSelf = href.includes('ruthub.com');
  const textIsImage = text.includes('<img');
  return `
  <a href="${href}" target="_blank"
    title="${title || (textIsImage ? href : text)}" 
    ${isSelf ? '' : 'rel="external nofollow noopener noreferrer"'}
  >${text}
  </a>`.replace(/\s+/g, ' ').replace('\n', '');
}
function imageParse(src, title, alt) {
  return `
  <br><a href="${src}" 
    target="_blank" rel="nofollow noopener noreferrer">
    <img src="${src}" title="${title || alt || ''}" 
      style="max-width:95%; max-height:45%"
      alt="${alt || title || src}"
    />
  </a><br>`.replace(/\s+/g, ' ').replace('\n', '');
}
function headingParse(text, level) {
  let realLevel = level + 2;
  return '<h' + realLevel + '>' + text + '</h' + realLevel + '>\n';
}
renderer.link = linkParse;
renderer.image = imageParse;
renderer.paragraph = paragraphParse;
renderer.heading = headingParse;
marked.setOptions({
  gfm: true,
  breaks: true,
  pedantic: false,
  smartLists: true,
  smartypants: true,
  renderer: renderer
})
// // helper for open and close  image popup
// function closeImgPopup() {
//   const mask = document.getElementById('image-popup')
//   if (mask) {
//     window.onscroll = null
//     mask.setAttribute('class', '')
//     setTimeout(() => {
//       mask && document.body.removeChild(mask)
//     }, 100)
//   }
// }
// function openImgPopup(src, className) {
//   if (!src) return false
//   const image = document.createElement('img')
//   image.src = src
//   className && image.setAttribute('class', className)

//   const oldMask = document.getElementById('image-popup')
//   oldMask && document.body.removeChild(oldMask)

//   const mask = document.createElement('div')
//   mask.setAttribute('id', 'image-popup')
//   mask.appendChild(image)
//   document.body.appendChild(mask)

//   setTimeout(() => {
//     mask.setAttribute('class', 'display')
//   }, 100)

//   window.onscroll = closeImgPopup
//   mask.onclick = closeImgPopup
// }

const PerPage = 20; 
const WPM = 300;
const mapFlag = {'1': 'Todo', '2': 'Doing', '3': 'Done', 'Options': 'Options'};
//## show dropdown
function showMenu() { showDrop("drop-menu"); }
function showOption() { showDrop("drop-opt"); }
function showDrop(id_name) {
  document.getElementById(id_name).classList.toggle("show");
}
const TOK = 'NoSeSNekoTr'; // for get cookie token
const IDENT = 'YITnEdIr'  // for get cookie identity
//## action once window loaded
window.addEventListener('load', function() {
  //# check if authed
  var iden = getCookie(IDENT);
  document.getElementById('login-link').style.display = iden ? 'none' : '';
  document.getElementById('menu-down').style.display = iden ? '' : 'none';
  document.getElementById('profile-link').href = iden ? '/me/p/' + iden : '#';
  //# if show rut|tag|item edit link
  var editlink = document.getElementById('edit-link');
  if (editlink) {
    var logged_uname = document.getElementById('edit-uname'); 
    var omg = getCookie("oMg");
    var check = logged_uname
      ? iden && (iden === logged_uname.innerText || omg === 'true') // for rut only
      : iden;                                                 // for tag|item
    editlink.style.display = check ? '' : 'none';
    var createlink = document.getElementById('create-link');
    if (createlink) {createlink.style.display = check ? '' : 'none';}
    var addurllink = document.getElementById('addurl-link');
    if (addurllink) {addurllink.style.display = omg === 'true' ? '' : 'none';}
  }
  //# for check rut star status
  var rutbtn = document.getElementById('rut-btn');
  if (rutbtn && iden) {
    var rutid = document.getElementById('rut-id').innerText;
    axios.get('/api/checkstarrut/' + iden + '/' + rutid)
    .then(function(resp) {
      var status = resp.data.message;
      rutbtn.innerText = status == 'star' ? 'UnStar' : 'Star'
    });
  }
  //# for rut md
  var ctEle = document.getElementById('raw-rut-content');
  if (ctEle) {
    ctEle.style.display = 'none';
    document.getElementById('md-rut-content').innerHTML = marked(ctEle.innerText);
  }
  //# for rut-credential md
  var cred = document.getElementById('raw-credential');
  if (cred) {
    cred.style.display = 'none';
    document.getElementById('md-credential').innerHTML = marked(cred.innerText);
  }
  //# for collect md
  for (let i = 1;; i++) {
    var cEle = document.getElementById(`raw-collect${i}`);
    if (!cEle) { break; }
    var mdCele = document.getElementById(`md-collect${i}`);
    cEle.style.display = 'none';
    mdCele.innerHTML = marked(cEle.innerText);
  }
  //# for calculate the read time
  var r = document.getElementById('rut-main-content');
  var c = document.getElementById('rut-collect-content');
  var rText = r ? r.innerText : '...';
  var cText = c ? c.innerText : '...';
  var readTime = Math.ceil(( rText.length + cText.length ) / WPM);
  var rTime = document.getElementById('read-time-tips');
  if (rTime) {rTime.innerText = 'about ' + readTime + ' min read';}
  //# for tag intro md
  var introEle = document.getElementById('raw-tag-intro');
  if (introEle) {
    document.getElementById('md-tag-intro').innerHTML = marked(introEle.innerText) || '...';
  }
  //# for check tag follow status
  var tagbtn = document.getElementById('tag-btn');
  //check tag star status
  if (tagbtn && iden) {
    var tagid = document.getElementById('tag-id').innerText;
    axios.get('/api/checkstartag/' + iden + '/' + tagid)
    .then(function(resp) {
      var status = resp.data.message;
      tagbtn.innerText = status == 'star' ? 'UnFollow' : 'Follow'
    });
  }
  //# for item detail md
  var detailEle = document.getElementById('raw-item-detail');
  if (detailEle) {
    document.getElementById('md-item-detail').innerHTML = marked(detailEle.innerText) || '...';
  }
  //# for check flag status
  var optbtn = document.getElementById('opt-btn');
  if (optbtn && iden) {
    var itemid = document.getElementById('item-id').innerText;
    axios.get('/api/checkflag/' + iden + '/' + itemid)
    .then(function(resp) { 
      var status = resp.data.message;
      optbtn.innerText = mapFlag[status]
    });
  }
  //# for author intro md
  var auIntroEle = document.getElementById('raw-au-intro');
  if (auIntroEle) {
    document.getElementById('md-au-intro').innerHTML = marked(auIntroEle.innerText) || '...';
  }
  //# for author page show item or rut or both
  var rutNumEle = document.getElementById('a-rut-num');
  var itemNumEle = document.getElementById('a-item-num');
  var auRutsDiv = document.getElementById('ruts-of-author');
  var auItemsDiv = document.getElementById('items-of-author');
  if (rutNumEle && itemNumEle && auRutsDiv && auItemsDiv) {
    var auRutNum = rutNumEle.innerText;
    var auItemNum = itemNumEle.innerText;
    auRutsDiv.style.display =  auRutNum && auRutNum !== '0'  ? '' : 'none';
    auItemsDiv.style.display =  auItemNum && auItemNum !== '0'  ? '' : 'none';
  }
});

// for show full content or detail or intro for rut,item,tag,author
function showFull(cSelector, btnSelector, cssName) {
  var full = document.getElementById(cSelector);
  var btn = document.getElementById(btnSelector);
  var ifShowMore = btn.innerText == 'more...' ? true : false
  full.className = ifShowMore ? '' : cssName;
  btn.innerText = ifShowMore ? 'less...' : 'more...';
}

function onSearch(selector, ty) {
  var q = document.getElementById(selector);
  if (q && q.value != "") {
    var openUrl = ty === "g" 
      ? 'https://www.google.com/search?q=site:ruthub.com/%20'
      : '/me/item/search?q=';
    window.open(openUrl + q.value, "_blank");
  }
}
var idxPage = 1;
var hasMoreIdx = true;
function loadMoreIndexRuts() {
  if (!hasMoreIdx) { return; }
  idxPage += 1;
  axios.get(`/moreindexruts/${idxPage}`)
  .then(function(resp) {
    let data = resp.data || "";
    if ( !Boolean(data) ) {
      console.log("No More");
      hasMoreIdx = false;
    }
    document.getElementById('ruts-in-home').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}
function followTag() {
  var tok = getCookie(TOK);
  var tagbtn = document.getElementById('tag-btn');
  if (tok && tagbtn) {
    axios.defaults.headers.common['Authorization'] = tok;
    var action = tagbtn.innerText == "Follow" ? 1 : 0;
    var tagid = document.getElementById('tag-id').innerText;
    axios.get(`/api/startag/${tagid}/${action}/love`)
    .then(function(resp){
      var status = resp.data.message;
      tagbtn.innerText = status == 'star' ? 'UnFollow' : 'Follow'
    });
  }
}
function starRut() {
  var tok = getCookie(TOK);
  var rutbtn = document.getElementById('rut-btn');
  if (tok && rutbtn) {
    axios.defaults.headers.common['Authorization'] = tok;
    var action = rutbtn.innerText == "Star" ? 1 : 0;
    var rutid = document.getElementById('rut-id').innerText;
    axios.get(`/api/starrut/${rutid}/${action}/love`)
    .then(function(resp){
      var status = resp.data.message;
      rutbtn.innerText = status == 'star' ? 'UnStar' : 'Star'
    });
  }
}
function flagItem(flag) {  // 1-todo 2-doing 3-done
  var tok = getCookie(TOK);
  var optbtn = document.getElementById('opt-btn');
  if (tok && optbtn) {
    axios.defaults.headers.common['Authorization'] = tok;
    var itemid = document.getElementById('item-id').innerText;
    axios.get(`/api/staritem/${itemid}/${flag}/0/love`)
    .then(function(resp) { 
      var status = resp.data.message;
      optbtn.innerText = mapFlag[status]
    });
  }
}
function getCookie(c_name) {
  var c_start = 0, c_end = 0, ck = document.cookie;
  if (ck.length > 0) {
    c_start = ck.indexOf(c_name + "=");
    if (c_start != -1) {
      c_start = c_start + c_name.length + 1;
      c_end = ck.indexOf(";", c_start);
      if (c_end === -1) { c_end = ck.length;}
    }
    var c = ck.substring(c_start,c_end);
    return unescape(c) 
  }
  return ""
}
// Close the dropdown menu if the user clicks outside of it
window.onclick = function(event) {
  if (!event.target.matches('.toolbtn')) {
    var dropdowns = document.getElementsByClassName("dropdown-content");
    var i;
    for (i = 0; i < dropdowns.length; i++) {
      var openDropdown = dropdowns[i];
      if (openDropdown.classList.contains('show')) {
        openDropdown.classList.remove('show');
      }
    }
  }
} 
// item page
var itemPage = 1;
function loadMoreInItem() {
  var data = '';
  var slug = document.getElementById('item-slug').innerText;
  var count = document.getElementById('i-rut-num').innerText;
  if (itemPage >= count / PerPage) { return; }
  itemPage += 1;
  axios.get(`/item/${slug}/${itemPage}`)
  .then(function(resp) {
    data = resp.data;
    document.getElementById('ruts-in-item').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}
// rut page
function shareWindow(to) {
  const curTitle = document.title;
  const pageUrl = window.location.href;
  switch (to) {
    case 'tw':
      share_url = `https://twitter.com/share?text=${curTitle}&url=${pageUrl}`;
      break
    case 'rd':
      share_url = `https://www.reddit.com/submit?url=${pageUrl}&title=${curTitle}`;
      break
    case 'fb':
      share_url = `https://www.facebook.com/sharer/sharer.php?u=${pageUrl}`;
      break
    case 'wb':
      share_url = `http://service.weibo.com/share/share.php?url=${pageUrl}&title=${curTitle}&source=${pageUrl}&sourceUrl=${pageUrl}&content=${curTitle}`;
      break
    case 'wx':
      share_url = `http://qr.topscan.com/api.php?text=${pageUrl}&w=300&el=h&m=10`;
      break
  }
  const url = encodeURI(share_url);
  const winName = 'newWin';
  const awidth = screen.availWidth / 2;
  const aheight = screen.availHeight / 5 * 2;
  const atop = (screen.availHeight - aheight) / 2;
  const aleft = (screen.availWidth - awidth) / 2;
  const param0 = 'scrollbars=0,status=0,menubar=0,resizable=2,location=0';
  const params = `top=${atop},left=${aleft},width=${awidth},height=${aheight},${param0}`;
  const win = window.open(url, winName, params);
  win.focus();
}
// tag page
var tagPage = 1;
function loadMoreInTag() {
  var data = '';
  var tagid = document.getElementById('tag-id').innerText;
  var count = document.getElementById('t-rut-num').innerText;
  if (tagPage >= count / PerPage) { return; }
  tagPage += 1;
  axios.get(`/tag/${tagid}/${tagPage}`)
  .then(function(resp) {
    data = resp.data;
    document.getElementById('ruts-w-tag').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}
// author page
var rutIdx = 1;
function loadMoreRutOfAuthor() {
  var data = '';
  var authorslug = document.getElementById('author-slug').innerText;
  var count = document.getElementById('a-rut-num').innerText;
  if (rutIdx >= count / PerPage) { return; }
  rutIdx += 1;
  axios.get(`/author/${authorslug}/${rutIdx}/rut`)
  .then(function(resp) {
    data = resp.data;
    document.getElementById('ruts-w-author').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}
var itemIdx = 1;
function loadMoreItemOfAuthor() {
  var data = '';
  var authorslug = document.getElementById('author-slug').innerText;
  var count = document.getElementById('a-item-num').innerText;
  if (itemIdx >= count / PerPage) { return; }
  itemIdx += 1;
  axios.get(`/author/${authorslug}/${itemIdx}/item`)
  .then(function(resp) {
    data = resp.data;
    document.getElementById('items-w-author').innerHTML += data;
  });
  window.scrollTo(0, document.body.scrollHeight);
}
