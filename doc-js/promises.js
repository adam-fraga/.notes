/* Create 2 callback for resolver & reject status */
function successCallback(résultat) {
  console.log("L'opération a réussi avec le message : " + résultat);
}

function failureCallback(erreur) {
  console.error("L'opération a échoué avec le message : " + erreur);
}

/* Some async function return a promise */
function faireQqc() {
  return new Promise((successCallback, failureCallback) => {
    console.log("C'est fait");
    // réussir une fois sur deux
    if (Math.random() > 0.5) {
      successCallback("Réussite");
    } else {
      failureCallback("Échec");
    }
  });
}

/* Then handle resolve & catch handle reject */

//1 Normal way
const promise = faireQqc();
promise.then(successCallback, failureCallback);

//2 Async call
faireQqc().then(successCallback, failureCallback);


