window.addEventListener('load', function() {
  //# for item md content
  var ctEle = document.getElementById('raw-item-content');
  if (ctEle) {
    ctEle.style.display = 'none';
    document.getElementById('md-item-content').innerHTML = marked(ctEle.innerText);
  }
});
