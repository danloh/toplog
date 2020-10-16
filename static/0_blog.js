let S_PREFIX = "new-b-";
let UP_BLOG = {aname: '', intro: '', blog_link: ''};

let IS_NEW_OR_NOT = true;  // new or edit

function buildBlog() {
  let ids = [
    'aname', 'avatar', 'intro', 'topic', 'blog_link','blog_host', 
    'gh_link', 'other_link', 'csrf'
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
  let csrf = vals[ids.indexOf('csrf')];

  let ckb = document.getElementById('new-b-is_top');
  // console.log(ckb.checked)
  let is_top = ckb && ckb.checked;
  
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
    is_top,
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
      'aname', 'avatar', 'intro', 'topic', 'blog_link', 
      'blog_host',  'gh_link', 'other_link'
    ];
    setValsByIDs(ids, S_PREFIX, res_blog);
    // init checkbox
    if (res_blog.is_top) {
      let ckbox = document.getElementById('new-b-is_top');
      if (ckbox) { ckbox.setAttribute('checked', true); }
    }

    // load tags and init tagsbar
    // await loadTagsInitBar('blog', res_blog.id);
  }

  initAutoSize(['new-b-intro', 'new-b-avatar']);

})
