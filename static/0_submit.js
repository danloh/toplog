let SUB_PREFIX = "subtl-";

function buildSubmit() {
  let ids = [
    'title','link','content','pub_at','author','topic','ty','logo','csrf'
  ];
  let vals = getValsByIDs(ids, SUB_PREFIX);
  let csrf = vals[ids.indexOf('csrf')];
  
  // refer to struct 
  let newSub = {
    title: vals[ids.indexOf('title')],
    content: vals[ids.indexOf('content')],
    logo: vals[ids.indexOf('logo')],
    author: vals[ids.indexOf('author')],
    ty: vals[ids.indexOf('ty')],
    topic: vals[ids.indexOf('topic')],
    link: vals[ids.indexOf('link')], 
    post_by: getCookie(IDENT),
    pub_at: vals[ids.indexOf('pub_at')],
  };
  return [newSub, csrf];
}

async function onSubmit() {
  if (!getCookie(TOK)) return;
  const bd = buildSubmit();
  let newSub = bd[0];
  let csrf = bd[1];
  
  let headers = {
    'Authorization': getCookie(TOK),
    'CsrfToken': csrf, 
    'Content-Type': 'application/json'
  };
  let options = {
    method:  'POST', 
    headers,
    body: JSON.stringify(newSub)
  };
  let resp = await fetch('/api/items', options);
  if (!resp.ok) {
    alert("Something failed");
    return;
  }
  // console.log(resp);
  let res_item = await resp.json();
  let itmid = res_item.id || UP_ITEM.id;
  if (!itmid) return;
  window.location.href = '/item/' + itmid;
}

document.addEventListener('DOMContentLoaded', async function() {
  if (!getCookie(TOK)) {
    let redirUrl = document.location.href;
    window.location.href = '/auth?to=signin&redirect=' + redirUrl;
  }
  // load information in query
  let query = decodeURIComponent(document.location.search);
  let url  = getQueryParam('l', query);
  let title  = getQueryParam('t', query);
  let imgUrl = getQueryParam('img', query);
  let desc  = getQueryParam('des', query);
  let pubat  = getQueryParam('d', query) || getToday();

  let info = {
    title: title,
    link: url,
    logo: imgUrl,
    content: desc,
    pub_at: pubat,
  };

  //initAutoSize(['subtl-title', 'subtl-link', 'subtl-intro', 'subtl-cover']);

  let ids = ['title', 'link', 'content', 'pub_at', 'logo'];
  setValsByIDs(ids, SUB_PREFIX, info);
})

function getToday() {
  let now = new Date();
  let dd = String(now.getDate()).padStart(2, '0');
  let mm = String(now.getMonth() + 1).padStart(2, '0'); //January is 0!
  let yyyy = String(now.getFullYear());
  let today = yyyy + '-' + mm + '-' + dd;
  return today;
}
