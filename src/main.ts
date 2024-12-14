let signIn = document.getElementById("y");
let signUp = document.getElementById("n");

/* @ts-ignore */
signIn.addEventListener("click", () => {
    document.location='./src/html/login'
})

// @ts-ignore
signUp.addEventListener("click", () => {
    document.location='./src/html/signup'
})