const title = document.getElementById('title');

setTimeout(() => {
    title.innerText = title.innerText + ' (changed)';
}, 2_500);
