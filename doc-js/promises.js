/* Create 2 callback for resolver & reject status */
function successCallback(data) {
  console.log("Donnée trouvé ! " + data);
}

function failureCallback(erreur) {
  console.error("L'opération a échoué avec le message : " + erreur);
}

function getData() {
  return new Promise((successCallback, failureCallback) => {
    //Simuling async operation from database or fetch api
    setTimeout(() => {
      //If donnée trouvé en base de donnée
      successCallback(data);
      //Else
      failureCallback(new Error("Donnée non trouvée"));
    }, 2000);
  });
}

//OR USE RESOLVE & REJECT

function getUser() {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      let data = { data: "data" };
      resolve(data);
    }, 2000);
  });
}

/* Then handle resolve & catch handle reject */
getData().then(successCallback, failureCallback);
getUser().then((user) => console.log(user));
