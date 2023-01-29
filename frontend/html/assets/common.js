// https://www.w3schools.com/colors/colors_names.asp
const COLORS = [
  "AliceBlue","AntiqueWhite","Aqua","Aquamarine","Azure","Beige","Bisque","Black","BlanchedAlmond","Blue","BlueViolet","Brown",
  "BurlyWood","CadetBlue","Chartreuse","Chocolate","Coral","CornflowerBlue","Cornsilk","Crimson","Cyan","DarkBlue","DarkCyan",
  "DarkGoldenRod","DarkGray","DarkGrey","DarkGreen","DarkKhaki","DarkMagenta","DarkOliveGreen","DarkOrange","DarkOrchid",
  "DarkRed","DarkSalmon","DarkSeaGreen","DarkSlateBlue","DarkSlateGray","DarkSlateGrey","DarkTurquoise","DarkViolet","DeepPink",
  "DeepSkyBlue","DimGray","DimGrey","DodgerBlue","FireBrick","FloralWhite","ForestGreen","Fuchsia","Gainsboro","GhostWhite",
  "Gold","GoldenRod","Gray","Grey","Green","GreenYellow","HoneyDew","HotPink","IndianRed","Indigo","Ivory","Khaki","Lavender",
  "LavenderBlush","LawnGreen","LemonChiffon","LightBlue","LightCoral","LightCyan","LightGoldenRodYellow","LightGray","LightGrey",
  "LightGreen","LightPink","LightSalmon","LightSeaGreen","LightSkyBlue","LightSlateGray","LightSlateGrey","LightSteelBlue",
  "LightYellow","Lime","LimeGreen","Linen","Magenta","Maroon","MediumAquaMarine","MediumBlue","MediumOrchid","MediumPurple",
  "MediumSeaGreen","MediumSlateBlue","MediumSpringGreen","MediumTurquoise","MediumVioletRed","MidnightBlue","MintCream",
  "MistyRose","Moccasin","NavajoWhite","Navy","OldLace","Olive","OliveDrab","Orange","OrangeRed","Orchid","PaleGoldenRod",
  "PaleGreen","PaleTurquoise","PaleVioletRed","PapayaWhip","PeachPuff","Peru","Pink","Plum","PowderBlue","Purple","RebeccaPurple",
  "Red","RosyBrown","RoyalBlue","SaddleBrown","Salmon","SandyBrown","SeaGreen","SeaShell","Sienna","Silver","SkyBlue","SlateBlue",
  "SlateGray","SlateGrey","Snow","SpringGreen","SteelBlue","Tan","Teal","Thistle","Tomato","Turquoise","Violet","Wheat","White",
  "WhiteSmoke","Yellow","YellowGreen"];

function getRandomColor() {
  const idx = Math.ceil(Math.random() * COLORS.length);
  return COLORS[idx];
}

function getParam(cname) {
    // console.log(document.location.search);
    let name = cname + "=";
    let ca = document.location.search.split('&');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        let p = c.indexOf(name);
        if (c.indexOf(name) > -1) {
            return decodeURIComponent(c.substring(p + name.length, c.length));
        }
    }
    return "";
}

// Cookie
function setCookie(cname, cvalue, exdays) {
  window.localStorage.setItem(cname, cvalue);
  // const d = new Date();
  // d.setTime(d.getTime() + (exdays*24*60*60*1000));
  // let expires = "expires="+ d.toUTCString();
  // document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/;SameSite=None;max-age=31536000";
  document.cookie = cname + "=" + cvalue + ";path=/;SameSite=lax;max-age=31536000";
}
function getCookie(cname) {
  const v = window.localStorage.getItem(cname);
  if (v)
    return v;
  let name = cname + "=";
  console.log('document.cookie='+document.cookie);
  let decodedCookie = decodeURIComponent(document.cookie);
  let ca = decodedCookie.split(';');
  for(let i = 0; i <ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) == ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}
const COOKIE_NAME_PLAYER_NAME = 'playerName';
// End

let popupModalEle = null;
function getModal() {
  const modalId = 'popupModal';
  const modalHeaderId = modalId + 'Header';
  const modalContentBodyId = modalId + 'Content';
    if (popupModalEle == null)
    popupModalEle = document.getElementById(modalId);
  if (popupModalEle == null) {
    const modalHeader = document.createElement('div');
    modalHeader.className = 'modal-header';
    modalHeader.innerHTML = '<span class="close">&times;</span><h2 id="' + modalHeaderId + '"></h2>';
    const modalBody = document.createElement('div');
    modalBody.className = 'modal-body';
    modalBody.innerHTML = '<p id="' + modalContentBodyId + '"></p>';
    const modalFooter = document.createElement('div');
    modalFooter.className = 'modal-footer';
    modalFooter.innerHTML = '<h3>&nbsp;</h3>';
    const modalContent = document.createElement('div');
    modalContent.className = 'modal-content';
    modalContent.appendChild(modalHeader);
    modalContent.appendChild(modalBody);
    modalContent.appendChild(modalFooter);
    const modal = document.createElement('div');
    modal.id = modalId;
    modal.className = 'modal';
    modal.appendChild(modalContent);
    document.body.appendChild(modal);
    modal.setAttribute('headerId', modalHeaderId);
    modal.setAttribute('contentId', modalContentBodyId);
    popupModalEle = modal;
  }
  return popupModalEle;
}
function popupModal(title, content, bgColor, callback) {
  const modal = getModal();
  document.getElementById(modal.getAttribute('headerId')).textContent = title;
  document.getElementById(modal.getAttribute('contentId')).textContent = content;
  modal.style.display = 'block';
}
function hideModal() {
  getModal().style.display = 'none';
}
// websocket
function connWebSocket(u, openCallback, closeCallback, messageCallback, errorCallback) {
  const ws = new WebSocket(u);
  // console.log(ws.readyState);
  if (errorCallback)
    ws.onerror = errorCallback;
  else
    ws.onerror = function(event) {};
  ws.onopen = openCallback;
  ws.onclose = closeCallback;
  ws.onmessage = function (event) {
      const m = event.data;
      if (typeof(m) === 'string') {
        console.log(m);
        if (m.length > 0)
          messageCallback(m);
      } else if (m instanceof ArrayBuffer) {}
  };
  return ws;
}
