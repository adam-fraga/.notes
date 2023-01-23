const list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const work_in_progress = async () => {
  await sleep(5000);
  console.log("async task...");
};

function linear_search(list, target) {
  console.log("Linear search is starting");
  for (let i in list) {
    if (list[i] === target) {
      console.log("Target is : " + list[i]);
      return 0;
    }
  }
}

/* (More effiscient on sorted element) */
function binary_search(list, target) {
  console.log("Binary search is starting");
  let n = list.length / 2;
  while (n !== target) {
    if (n > target) {
      n = n / 2;
    } else if (n < target) {
      n = n + 1;
    }
  }
  console.log("Target is : " + n);
}

function recursive_binary_search(list, target, nb = list.length / 2) {
  if (nb == target) {
    console.log("Target is: " + nb);
  } else if (nb > target) {
    recursive_binary_search(list, target, nb / 2);
  } else if (nb < target) {
    recursive_binary_search(list, target, nb + 1);
  } else {
    recursive_binary_search(list, target, nb);
  }
}

linear_search(list, 8);
binary_search(list, 2);

console.log("Recursive Binary search is starting");
recursive_binary_search(list, 6);
