let S_PREFIX = "new-b-";
let UP_BLOG = {aname: '', intro: '', blog_link: ''};

let IS_NEW_OR_NOT = true;  // new or edit

function buildBlog() {
  let ids = [
    'aname', 'avatar', 'intro', 'topic', 'blog_link','blog_host', 
    'gh_link', 'other_link', 'is_top', 'csrf'
  ];
  let vals = getValsByIDs(ids, S_PREFIX);

  let aname = vals[ids.indexOf('aname')];
  let avatar = vals[ids.indexOf('avatar')];
  let intro = vals[ids.indexOf('intro')];
  let topic = vals[ids.indexOf('topic')];
  let blog_host = vals[ids.indexOf('blog_host')];
  let blog_link = vals[ids.indexOf('blog_link')];
  let gh_link = vals[ids.indexOf('gh_link')];
  let other_link = vals[ids.indexOf('other_link')];
  let is_top = vals[ids.indexOf('is_top')];
  let csrf = vals[ids.indexOf('csrf')];

  console.log("is top", is_top);
  
  // refer to struct NewBlog
  let new_blog = {
    aname,
    avatar,
    intro,
    topic,
    blog_link,
    blog_host,
    gh_link,
    other_link,
    is_top: is_top == 'on' ? true : false,
  };
  // console.log(new_blog);
  return [new_blog, csrf];
}

async function newBlog() {
  if (!getCookie(TOK)) return;
  let bd = buildBlog();
  let new_blg = bd[0];
  let csrf = bd[1];
  let aname = new_blg.aname;
  let blogLink = new_blg.blog_link;


  if (aname && blogLink && csrf) {
    let newaBtn = document.getElementById(S_PREFIX + "btn");
    if (newaBtn) { newaBtn.innerHTML = "Processing"; }

    // if update, check the change
    let IF_TO_FETCH = true;
    if (!IS_NEW_OR_NOT
      && UP_BLOG.aname == aname 
      && UP_BLOG.avatar == new_blg.avatar
      && UP_BLOG.intro == new_blg.intro
      && UP_BLOG.blog_link == new_blg.blog_link
      && UP_BLOG.blog_host == new_blg.blog_host
      && UP_BLOG.gh_link == new_blg.gh_link
      && UP_BLOG.other_link == new_blg.other_link
      && UP_BLOG.topic == new_blg.topic
      && UP_BLOG.is_top == new_blg.is_top
    ) { 
      IF_TO_FETCH = false;
      if (delTagList.length == 0 && addTagList.length == 0) {
        alert('Nothing Changed');
        return;
      }
    }

    let q_data = IS_NEW_OR_NOT 
      ? new_blg
      : Object.assign({id: UP_BLOG.id}, new_blg);

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
      ? await fetch('/api/blogs', options) 
      : {ok: true, nofetch: true};
    // console.log(resp);

    if (!resp.ok) {
      alert("Something failed");
      return;
    }
    // console.log(resp);
    let res_blog = resp.nofetch ? {} : await resp.json();

    // edit tag
/*     if (addTagList.length > 0) {
      let addTags = {
        tnames: addTagList,
        blog_id: res_blog.id || UP_BLOG.id,
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
        blog_id: res_blog.id || UP_BLOG.id,
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

    let name = res_blog.aname;
    let b64_name = Base64encode(name, true);
    window.location.href = '/from?by=' + b64_name;
  } else {
    alert("Invalid Input", aname, blogLink, csrf);
    console.log("Invalid Input", aname, blogLink, csrf);
    return;
  }
}

document.addEventListener('DOMContentLoaded', async function() {
  if (!getCookie(TOK)) return;
  let docPath = document.location.pathname;
  if (docPath.startsWith('/editblog')) {
    // load blog
    let bid = document.location.search.split('?id=')[1];
    if (!bid) return;
    let resp = await fetch(`/api/blogs/${bid}`);
    if (!resp.ok) return;
    let res_blog = await resp.json();

    UP_BLOG = res_blog;
    IS_NEW_OR_NOT = false;

    let ids = [
      'aname', 'avatar', 'intro', 'topic', 'blog_link','blog_host', 
      'gh_link', 'other_link', 'is_top'
    ];
    setValsByIDs(ids, S_PREFIX, res_blog);
    // load tags and init tagsbar
    // await loadTagsInitBar('blog', res_blog.id);
  }

  // initAutoSize(['new-b-intro', 'new-b-avatar']);

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
