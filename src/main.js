const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let gifListEl;
let lastSearchString = '';

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  gifListEl = document.querySelector("#gif-list");

  greetInputEl.focus();
  // Enter Key clicks the button
  greetInputEl.addEventListener("keypress", function(e) {
    if (e.key == 'Enter') {
      e.preventDefault();
      document.getElementById('search-button').click();
    }
  });
});



async function searchGif() {
  lastSearchString = greetInputEl.value;
  let gifs = await invoke("search_gif", { name: greetInputEl.value });
  // clear all the previous elements
  gifListEl.replaceChildren();
  for (let i = 0; i < gifs.length; i++) {
    const div = document.createElement('div');
    div.setAttribute('class', 'column');

    const img = document.createElement('img');
    img.src = gifs[i];
    img.setAttribute('class', 'gif-img');
    img.addEventListener('click', copyToClipboard);
    div.appendChild(img);
    gifListEl.appendChild(div);
    console.log(gifs[i]);
    document.querySelector('#result-message').innerHTML = ''
  }
}

async function copyToClipboard(e) {
  console.log(e)
  console.log(e.target)
  console.log(e.target.currentSrc)
  let resultString = await invoke("copy_to_clipboard", { name: lastSearchString, url: e.target.currentSrc });
  console.log(resultString)
  document.querySelector('#result-message').innerHTML = resultString;
}


window.searchGif = searchGif;
window.copyToClipboard = copyToClipboard;
