import init, { replace_name_with_emoji,replace_emoji_with_name } from './emojikingdom.js';

const moveEle = document.getElementById('move');
moveEle.innerText = "🤖"
const animation = {start: 0,end: document.body.offsetHeight- moveEle.offsetHeight,duration: 10000};
const emojis = ["🥱","💩","🏋️","🙌","💰","🍜","💤","💪","🐪","🐫"]
function animaEle() {
    const startTime = Date.now();
    moveEle.innerText =  emojis[Math.floor(Math.random() * emojis.length)];
    const updatePosition = () => {
        const currentTime = Date.now();
        const elapsedTime = currentTime - startTime;
        const yProgress = Math.min(elapsedTime / animation.duration, 1);
        const yPosition = animation.start + (animation.end - animation.start) * yProgress;
        moveEle.style.transform = `translate(0px, ${yPosition}px)`;
        if (elapsedTime < animation.duration || elapsedTime < animation.duration) {
            requestAnimationFrame(updatePosition);
        } else {
            animaEle();
        }
    };
    requestAnimationFrame(updatePosition);
}
animaEle();

const explain_mode = document.getElementById("explain_mode");
const to_emoji_btn =  document.getElementById("to_emoji_btn");
const to_name_btn = document.getElementById('to_name_btn');
const to_jp_name_btn = document.getElementById('to_jp_name_btn');
const input_area = document.getElementById('input_area');

window.addEventListener("load",(event)=>{
    if(localStorage.getItem("dark_mode")==="dark") {
        document.body.classList.add("dark");
        input_area.classList.add("dark");
    }
});
document.getElementById("light_btn").addEventListener("click",(event)=>{
    localStorage.removeItem("dark_mode");
    document.body.classList.remove("dark");
    input_area.classList.remove("dark");
});
document.getElementById("dark_btn").addEventListener("click",(event)=>{
    localStorage.setItem("dark_mode","dark");
    document.body.classList.add("dark");
    input_area.classList.add("dark");
});


async function run() {
    await init();
    to_emoji_btn.addEventListener("click", (event) => {
        console.log(explain_mode.checked)
        input_area.value = replace_name_with_emoji(input_area.value,explain_mode.checked);
    });
    to_name_btn.addEventListener("click", (event) => {
        input_area.value = replace_emoji_with_name(input_area.value,0,explain_mode.checked);
    });
    to_jp_name_btn.addEventListener("click", (event) => {
        input_area.value = replace_emoji_with_name(input_area.value,1,explain_mode.checked);
    });
}
run();
