// CALLBACK HELL
getUser(21, (user) => {
  console.log(user);
  // We need User data to retrieve post so we need this imbrication
  getUserPosts(21, (posts) => {
    console.log(posts);

    // We need Post data to retrieve comment so we need this imbrication
    getPostComments(1, (comments) => {
      console.log(comments);
    });
  });
});

function getUser(id, callback) {
  setTimeout(() => {
    //Database query or fetch or whatever
    console.log("Getting User...");

    callback({ id: id, name: "Adam" });
  }, 2000);
}

function getUserPosts(userId, callback) {
  setTimeout(() => {
    //Database query or fetch or whatever
    console.log("Getting users posts in database for user: " + userId);
    let posts = ["post1", "post2", "..."];

    callback(posts);
  }, 2000);
}

function getPostComments(postId, callback) {
  setTimeout(() => {
    //Database query or fetch or whatever
    console.log("Retrieve comments for post id: " + postId);
    let comments = ["comm1", "comm2", "comm3"];

    callback(comments);
  }, 2000);
}
