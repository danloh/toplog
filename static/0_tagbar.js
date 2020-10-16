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
