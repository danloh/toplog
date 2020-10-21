let S_PREFIX = "new-i-";
let NEW_AS = '';
let NEW_FOR = '';
let UP_ITEM = {title: '', content: '', logo: ''};

let IS_NEW_OR_NOT = true;  // new or edit

function buildItem () {
  let ids = ['title', 'content', 'logo', 'author', 'ty','topic', 'link', 'pub_at', 'csrf'];
  let vals = getValsByIDs(ids, S_PREFIX);

  let title = vals[ids.indexOf('title')];
  let content = vals[ids.indexOf('content')];
  let topic = vals[ids.indexOf('topic')] || NEW_FOR;
  let ty = vals[ids.indexOf('ty')] || NEW_AS || 'Article';
  let logo = vals[ids.indexOf('logo')];
  let author = vals[ids.indexOf('author')];
  let link = vals[ids.indexOf('link')];
  let pub_at = vals[ids.indexOf('pub_at')] || getToday();
  let csrf = vals[ids.indexOf('csrf')];
  
  // refer to struct NewItem
  let new_item = {
    title,
    content,
    logo,
    author,
    ty,
    topic,
    link,
    post_by: getCookie(IDENT),
    pub_at
  };
  // console.log(new_item);
  return [new_item, csrf];
}

async function newItem() {
  if (!getCookie(TOK)) return;
  let bd = buildItem();
  let new_itm = bd[0];
  let csrf = bd[1];
  let title = new_itm.title;
  let topic = new_itm.topic;
  let content = new_itm.content;

  if (title && topic && content && csrf) {
    let newaBtn = document.getElementById(S_PREFIX + "btn");
    if (newaBtn) { newaBtn.innerHTML = "Processing"; }

    // if update, check the change
    let IF_TO_FETCH = true;
    if (!IS_NEW_OR_NOT
      && UP_ITEM.title == title 
      && UP_ITEM.content == content
      && UP_ITEM.logo == new_itm.logo
      && UP_ITEM.author == new_itm.author
      && UP_ITEM.topic == topic
      && UP_ITEM.ty == new_itm.ty
      && UP_ITEM.link == new_itm.link
      && UP_ITEM.pub_at == new_itm.pub_at
    ) { 
      IF_TO_FETCH = false;
      if (delTagList.length == 0 && addTagList.length == 0) {
        alert('Nothing Changed');
        return;
      }
    }

    let q_data = IS_NEW_OR_NOT 
      ? new_itm
      : Object.assign({id: UP_ITEM.id}, new_itm);

    let headers = {
      'Authorization': getCookie(TOK),
      'CsrfToken': csrf, 
      'Content-Type': 'application/json'
    };

    let options = {
      method:  IS_NEW_OR_NOT ? 'POST' : 'PUT', 
      headers,
      body: JSON.stringify(q_data)
    };
    let resp = IF_TO_FETCH 
      ? await fetch('/api/items', options)
      : {ok: true, nofetch: true};
    // console.log(resp);

    if (!resp.ok) {
      alert("Something failed");
      return;
    }
    // console.log(resp);
    let res_item = resp.nofetch ? {} : await resp.json();

    // edit tag
/*     if (addTagList.length > 0) {
      let addTags = {
        tnames: addTagList,
        item_id: res_item.id || UP_ITEM.id,
        method: 1,
      };
      await fetch('/api/topics', 
        {
          method: 'PATCH',
          headers,
          body: JSON.stringify(addTags)
        }
      );
    }
    if (delTagList.length > 0) {
      let delTags = {
        tnames: delTagList,
        item_id: res_item.id || UP_ITEM.id,
        method: 0,
      };
      await fetch('/api/topics', 
        {
          method: 'PATCH',
          headers,
          body: JSON.stringify(delTags)
        }
      );
    } */

    let itmid = res_item.id || UP_ITEM.id;
    if (!itmid) return;
    window.location.href = '/item/' + itmid;
  } else {
    alert("Invalid Input");
    return;
  }
}

async function newItemViaUrl() {
  if (!getCookie(TOK)) return;
  let urlBtn = document.getElementById('new-i-url-btn');
  if (urlBtn) { urlBtn.innerHTML = "Processing"; }

  let urlEle = document.getElementById('new-i-viaurl');
  let url = urlEle ? urlEle.value : '';
  if (url.length < 1) return;

  let csrfTok = document.getElementById('new-i-csrf');
  let csrf = csrfTok ? csrfTok.value : ''
  if (!csrf) return;

  let sp_data = {
    url,
    topic: NEW_FOR || '',
    ty: NEW_AS || ''
  };

  let options = {
    method: 'PUT', 
    headers: { 
      'Authorization': getCookie(TOK), 
      'CsrfToken': csrf, 
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(sp_data)
  };
  let resp = await fetch('/api/spider', options);

  if (!resp.ok) {
    alert("Something failed");
    return;
  }
  // console.log(resp);
  let res_item = await resp.json();
  window.location.href = '/item/' + res_item.id;
}

// hide or show viaurl
function switchForm(to=0) {
  let viaurl = document.getElementById('new-i-form-viaurl');
  let viaform = document.getElementById('new-i-form-page');
  if (viaurl && viaform) { 
    viaurl.style.display = to ? 'none' : '';
    viaform.style.display = to ? '' : 'none';
  }
}

document.addEventListener('DOMContentLoaded', async function() {
  if (!getCookie(TOK)) return;
  let urlPath = document.location.pathname;
  let urlQry = document.location.search;
  if (urlPath.startsWith('/edititem')) {
    // load item
    let itmid = urlQry.split('?id=')[1];
    if (!itmid) return;
    let resp = await fetch(`/api/items/${itmid}`);
    if (!resp.ok) return;
    let res_item = await resp.json();

    UP_ITEM = res_item;
    IS_NEW_OR_NOT = false;

    let ids = ['title', 'content', 'logo', 'author', 'ty','topic', 'link', 'pub_at'];
    setValsByIDs(ids, S_PREFIX, res_item);
    
    initAutoSize(['new-i-content', 'new-i-link', 'new-i-logo']);

    // load tags and init tagsbar
    // await loadTagsInitBar('item', res_item.id);
  } else { // newitem
    let viaform = document.getElementById('new-i-form-page');
    if (viaform) { viaform.style.display = 'none'; }

    NEW_AS = getQueryParam('as', urlQry) || '';
    NEW_FOR = getQueryParam('for', urlQry) || '';
    // console.log(NEW_AS, NEW_FOR);
    let newids = ['ty','topic'];
    setValsByIDs(newids, S_PREFIX, {ty: NEW_AS, topic: NEW_FOR});
  }
})
