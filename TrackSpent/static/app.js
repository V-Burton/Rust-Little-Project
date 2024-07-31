function login() {
    const username = document.getElementById('login-username').value;
    const password = document.getElementById('login-password').value;
    fetch('/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    })
    .then(response => response.json())
    .then(data => {
        const loginMsg = document.getElementById('login-msg');
        if (data === "Login successful") {
            loginMsg.innerText = "";
            document.querySelector('.login').style.display = 'none';
            document.querySelector('.dashboard').style.display = 'block';
        } else {
            loginMsg.innerText = "Invalid username or password";
        }
    });
}

function register() {
    const username = document.getElementById('register-username').value;
    const password = document.getElementById('register-password').value;
    fetch('/register', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    })
    .then(response => response.json())
    .then(data => {
        const registerMsg = document.getElementById('register-msg');
        registerMsg.innerText = data;
    });
}

function showRegister() {
    document.querySelector('.login').style.display = 'none';
    document.querySelector('.register').style.display = 'block';
}

function showLogin() {
    document.querySelector('.register').style.display = 'none';
    document.querySelector('.login').style.display = 'block';
}

function sort() {
    document.querySelector('.description').style.display = 'block';
    document.querySelector('.categories').style.display = 'none';
}

function showCategories() {
    document.querySelector('.categories').style.display = 'block';
    document.querySelector('.description').style.display = 'none';
}

function showHome() {
    document.querySelector('.description').style.display = 'none';
}
