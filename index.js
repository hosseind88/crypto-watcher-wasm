import init, {
    get_coins_data,
    get_coins_list
} from "./pkg/cryptowatcherwasm.js";

window.onload = () => {
    init().then(async () => {
        try {
            await refreshList()

            setInterval(() => {
                refreshList()
            }, 5000);
        } catch (e) {}
    });
}

async function refreshList(tokens = "") {
    await get_coins_data(tokens).then((res) => {
        resetUI();
        res.data.forEach(item => setContent(item));
        hideLoading();
    });
}

function hideLoading() {
    if (document.querySelector('.loading')) {
        document.querySelector('.loading').classList.add('hidden')
    }
}

function resetUI() {
    document.body.innerHTML = ''
}

function setContent(data) {
    const box = document.createElement('div');
    box.classList.add('coin')
    const name = document.createElement('span');
    const price = document.createElement('span');
    const image = document.createElement('img');
    image.src = data.image.large
    name.innerText = data.name
    price.innerText = data.market_data.current_price.usd

    box.appendChild(image)
    box.appendChild(name)
    box.appendChild(price)
    document.body.appendChild(box);
}