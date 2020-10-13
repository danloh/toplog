let S_PREFIX = "new-i-";
let UP_ITEM = {title: '', intro: '', cover: ''};

let IS_NEW_OR_NOT = true;  // new or edit

function buildItem () {
  let ids = ['title', 'content', 'logo', 'author', 'ty','topic', 'link', 'pub_at', 'csrf'];
  let vals = getValsByIDs(ids, S_PREFIX);

  let title = vals[ids.indexOf('title')];
  let content = vals[ids.indexOf('content')];
  let topic = vals[ids.indexOf('topic')];
  let ty = vals[ids.indexOf('ty')] || 'Article';
  let logo = vals[ids.indexOf('logo')];
  let author = vals[ids.indexOf('author')];
  let link = vals[ids.indexOf('link')];
  let pub_at = vals[ids.indexOf('pub_at')];
  let csrf = vals[ids.indexOf('csrf')];
  
  // refer to struct NewItem
  let new_item = {
    title,
    slug: '',
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
      && UP_ITEM.topic == topic
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

    let sslug = res_item.slug || UP_ITEM.slug;
    if (!sslug) return;
    window.location.href = '/item/' + sslug;
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
    topic: '',
    ty: ''
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
  window.location.href = '/item/' + res_item.slug;
}

// hide or show viaurl
let showViaUrl = true;
function showInputUrl(mut=true) {
  showViaUrl = !showViaUrl && mut;
  let viaurl = document.getElementById('new-s-form-viaurl');
  if (viaurl) { viaurl.style.display = showViaUrl ? '' : 'none'; }
}

document.addEventListener('DOMContentLoaded', async function() {
  if (!getCookie(TOK)) return;
  let docPath = document.location.pathname;
  if (docPath.startsWith('/newitem')) {
    // = '_new_item_cache_';
  } else {
    // load item
    let slug = document.location.search.split('?slug=')[1];
    if (!slug) return;
    let resp = await fetch(`/api/items/${slug}`);
    if (!resp.ok) return;
    let res_item = await resp.json();

    UP_ITEM = res_item;
    IS_NEW_OR_NOT = false;

    let ids = ['title', 'content', 'logo', 'author', 'ty','topic', 'link', 'pub_at'];
    setValsByIDs(ids, S_PREFIX, res_item);
    // load tags and init tagsbar
    // await loadTagsInitBar('item', res_item.id);
  }

  initAutoSize(['new-i-title', 'new-i-content', 'new-i-logo']);

})


// autosize textarea
const newEvtListener = (parent, type, listener) => parent.addEventListener(type, listener);
function initAutoSize(ids=[]) {
  const autoSize = (id) => {
    let text = document.getElementById(id);
    const resize = () => {
        text.style.height = 'auto';
        text.style.height = text.scrollHeight + 'px';
    };
    const delayedResize = () => {
        window.setTimeout(resize, 0);
    };
    newEvtListener(text, 'change',  resize);
    newEvtListener(text, 'focus',  resize);
    newEvtListener(text, 'cut',     delayedResize);
    newEvtListener(text, 'paste',   delayedResize);
    newEvtListener(text, 'drop',    delayedResize);
    newEvtListener(text, 'keydown', delayedResize);

    text.focus();
    text.select();
    resize();
  };

  for (let id of ids) {
    autoSize(id);
  }
}

// load tags and init tag bar
//
let showTagList = [];
let addTagList = [];  // new added 
let delTagList = [];  // to be deled tags

function addTag(e, id) {
  if(e.keyCode === 13){
    e.preventDefault();
    let tagInput = document.getElementById(id);
    let tagName = tagInput ? tagInput.value : '';
    if (tagName.length > 0 && showTagList.indexOf(tagName) == -1) { 
      showTagList.push(tagName);
      // console.log(showTagList);
      tagInput.value = '';
    };
    if (tagName.length > 0 && addTagList.indexOf(tagName) == -1) { 
      addTagList.push(tagName);
      // console.log(addTagList);
      tagInput.value = '';
    };

    initTagBar(showTagList);
  }
}

function delTag(tag) {
  showTagList.splice(showTagList.indexOf(tag), 1);
  // console.log(showTagList);
  if (tag.length > 0 && delTagList.indexOf(tag) == -1) { 
    delTagList.push(tag);
    // console.log(delTagList);
  };
  initTagBar(showTagList);
}

function initTagBar(tags) {
  let container = document.getElementById('sa-tags-container');
  container.innerHTML = '';

  tags.forEach(tagName => {
    // add html element
    let tagSpan = document.createElement('span');
    tagSpan.className = "new-form-tag";
    tagSpan.innerHTML = `<span class="tag-name">${tagName}</span>`;
    let tagButton = document.createElement('a');
    tagButton.className = "edit-tag-btn";
    tagButton.innerHTML = " x";
    tagButton.href = 'javascript:void(0);';
    tagButton.onclick = () => delTag(tagName);
    tagSpan.appendChild(tagButton);
    container.appendChild(tagSpan);
  })
}

// async function loadTagsInitBar(pper, id) {
//   let resp_tags = await fetch(`/api/topics/${pper}?per=${id}&ext=0&page=1&perpage=${PerPage}`)
//   let res_tags = await resp_tags.json();
//   let tags = res_tags.topics.map(tpc => tpc.tname)
//   showTagList.push(...tags);
//   initTagBar(showTagList);
// }
