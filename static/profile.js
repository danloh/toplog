let uname;
let extKw;
let page = 1;
let totalCount;
let hasMore = false;

document.addEventListener('DOMContentLoaded', async function() {
  let srcSpan = document.getElementById('avatar-src');
  let src = srcSpan ? srcSpan.innerText : '';
  let name = document.getElementById('avatar-name');
  uname = name ? name.innerText : '';
  initAvatar('user-avatar', src, 180, uname);
  await navTo('vote', 'rl');
})

// submit/vote
async function navTo(kw, id) {
  extKw = kw;
  await loadAndAppend(kw);
  // active tab
  let active = document.getElementById(id);
  let tabs = document.getElementsByClassName('s-nav');
  for (let tab of tabs ) { tab.classList.remove("active-tab"); }
  active.classList.add("active-tab");
}

async function loadMoreAny() {
  if (!hasMore) return;
  page += 1;
  await loadAndAppend(extKw, true);
}

// load list and generate html then append to page
async function loadAndAppend(action, isMore=false) {
  let url = `/api/getitems/user?per=${uname}&kw=${action}&page=${page}&perpage=${PerPage}`;
  let resp = await fetch(url);
  if (!resp.ok) return;
  let res = await resp.json();
  let objs = res.items;
  if (page == 1) { totalCount = res.count; }
  hasMore = page <= Math.floor(totalCount / PerPage);

  let moreBtn = document.getElementById('loadmore-btn');
  if (moreBtn && !hasMore) { 
    moreBtn.setAttribute('disabled', true); 
    moreBtn.style.display = 'none';
  }

  let sList = document.getElementById('nav-list-box');
  if (!isMore) { sList.innerHTML = ''; }
  
  for (let obj of objs) {
    let sum = document.createElement('section');
    sum.className = 's-sum-info';
    let title = obj.title;
    let intro = obj.content;
    let link = obj.link || '/item/' + obj.slug;
    let inner = `
      <a class="title-link" href="${link}" target="_blank" rel="noopener">
        <b class="title">${title}</b>
      </a>
      <div class="s-sum">${intro}</div>
    `;
    sum.innerHTML = inner;
    sList.appendChild(sum);
  }
}
