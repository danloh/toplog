
let RedirectURL = '/';
document.addEventListener('DOMContentLoaded', async function() {
  if (getCookie(TOK)) {
    let authBox = document.getElementById('auth-box');
    if (authBox) { authBox.style.display = 'none'; }
  }
  let query = document.location.search;
  let docRefer = document.referrer;  // TODO: do some check 
  RedirectURL = getRedirect('redirect', query) || docRefer || '/';
  let toWhat = getQueryParam('to', query);
  let toNum = toWhat == 'signin' 
    ? 0 
    : toWhat == 'signup' 
      ? 1 
      : toWhat == 'reset' 
        ? 2 
        : toWhat == 'changepsw' 
          ? 3
          : toWhat == 'update' ? 4 : 5;

  // redirect when authed on signin / signup
  if (toNum < 2 && getCookie(TOK)) {
    window.location.href = RedirectURL.search('/auth?to=signin') == -1 ? RedirectURL : '/';
  }

  if (toNum >= 3 && !getCookie(TOK)) return;
  if (toNum == 4) {
    let name = document.getElementById('hide-uname');
    if (!name) return;
    let uname = name.value;
    if (uname != getCookie(IDENT)) return;
    // set user
    let getUser = await fetch(`/api/users/${uname}`);
    if (!getUser.ok) return;
    let userRes = await getUser.json();
    let userInfo = Object.assign({ },userRes.user);
    // console.log(userInfo);
    let ids = ['nickname', 'avatar', 'email', 'location', 'intro'];
    setValsByIDs(ids, 'auth-up-', userInfo);
  } 

  onSwitch(toNum);
  let titl = document.getElementById('auth-form-title');
  if (titl) {
    titl.innerText = toNum < 2
      ? 'Welcome' 
      : toNum == 2 
        ? 'Recover Password'
        : toNum == 3 
          ? 'Change Password'
          : 'Update Profile';
  }
}, false)

async function login() {
  let ids = ['username', 'psw'];
  let loginfo = getValsByIDs(ids, 'auth-');
  let uname = loginfo[ids.indexOf('username')];
  let password = loginfo[ids.indexOf('psw')];
  if (uname.trim().length < 3 || password.length < 8 ) {
    alert('Invalid Input')
  }
  let authData = { 
    uname: uname.trim(), 
    password: Base64encode(password, true)
  };
  let options = {
    method:  'POST', 
    headers: {'Content-Type': 'application/json'}, 
    body: JSON.stringify(authData)
  };
  let authResp = await fetch('/api/signin', options);
  if (!authResp.ok) { alert("Failed.."); return }

  let auth = await authResp.json();
  setAuth(auth);
  window.location.href = RedirectURL;
}

async function signup() {
  let ids = ['newuser', 'newpsw', 'repsw'];
  let loginfo = getValsByIDs(ids, 'auth-');
  let uname = loginfo[ids.indexOf('newuser')];
  let password = loginfo[ids.indexOf('newpsw')];
  let confirm = loginfo[ids.indexOf('repsw')];
  if (!regName.test(uname.trim()) || !regPsw.test(password)) {
    alert('Invalid Input'); 
    return
  }
  if (password !== confirm) { alert('Password Not Match'); return}

  let authData = { 
    uname: uname.trim(), 
    email: '',
    password: Base64encode(password, true),
    confirm: Base64encode(password, true),
    agree: true,
  };
  let options = {
    method:  'POST', 
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify(authData)
  };
  let regResp = await fetch('/api/signup', options);
  if (!regResp.ok) { alert("Failed.."); return }
  window.location.href = '/auth?to=signin';
}

function reset() { /*TODO*/ }

async function changePsw() {
  if (!getCookie(TOK)) return;
  let ids = ['oldpsw', 'chpsw', 'rechpsw'];
  let info = getValsByIDs(ids, 'auth-');
  let oldPsw = info[ids.indexOf('oldpsw')];
  let newPsw = info[ids.indexOf('chpsw')];
  let confirm = info[ids.indexOf('rechpsw')];
  if (!regPsw.test(oldPsw) || !regPsw.test(newPsw) || newPsw != confirm) {
    alert('Invalid Input'); 
    return
  }
  let uname = getCookie(IDENT);
  let pswData = { 
    old_psw: Base64encode(oldPsw, true),
    new_psw: Base64encode(newPsw, true),
    uname,
  };
  let options = {
    method:  'PUT', 
    headers: {
      'Content-Type': 'application/json', 
      'Authorization': getCookie(TOK),
    },
    body: JSON.stringify(pswData)
  };
  let chResp = await fetch('/api/users/' + uname, options);
  if (!chResp.ok) { alert("Failed.."); return }
  signOut('/auth?to=signin');
}

const regPsw = /^[\w#@~%^$&*-]{8,18}$/;
//const regPsw = /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[#@!~%^$&*-])[a-zA-Z\d#@!~%^$&*-]{8,18}$/;
const regEmail = /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/;
const regName = /^[\w-]{3,16}$/;

let trigger = 0;  // 0- login, 1- signup, 2- reset 3- change psw 4- update
function onSwitch(t=0) {
  trigger = t;
  let signupForm = document.getElementById('sign-up-form');
  let signinForm = document.getElementById('sign-in-form');
  let resetForm = document.getElementById('reset-form');
  let chpswForm = document.getElementById('chpsw-form');
  let upForm = document.getElementById('update-form');
  if (signinForm) { signinForm.style.display = trigger == 0 ? '' : 'none'; }
  if (signupForm) { signupForm.style.display = trigger == 1 ? '' : 'none'; }
  if (resetForm) { resetForm.style.display = trigger == 2 ? '' : 'none'; }
  if (chpswForm) { chpswForm.style.display = trigger == 3 ? '' : 'none'; }
  if (upForm) { upForm.style.display = trigger == 4 ? '' : 'none'; }
}

function setAuth(reg) {
  setCookie(TOK, reg.token, {expires: reg.exp});
  setCookie(IDENT, reg.user.uname, {expires: reg.exp});
  //setCookie(CAN, reg.user.permission, {expires: reg.exp});
  setCookie('oMg', reg.omg, {expires: reg.exp});
}

async function updateUser() {
  if (!getCookie(TOK)) return;
  let ids = ['nickname', 'avatar', 'email', 'location', 'intro'];
  let info = getValsByIDs(ids, 'auth-up-');
  let uname = getCookie(IDENT);
  let upUser = {
    uname,
    avatar: info[ids.indexOf('avatar')],
    email: info[ids.indexOf('email')],
    intro: info[ids.indexOf('intro')],
    location: info[ids.indexOf('location')],
    nickname: info[ids.indexOf('nickname')],
  };
  let options = {
    method:  'POST', 
    headers: {
      'Content-Type': 'application/json', 
      'Authorization': getCookie(TOK),
    },
    body: JSON.stringify(upUser)
  };
  let upResp = await fetch(`/api/users/${uname}`, options);
  if (!upResp.ok) return;

  window.location.href = '/@' + uname;
}
