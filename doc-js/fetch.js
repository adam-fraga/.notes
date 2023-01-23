const API_KEY_V4 =
  "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJlYzE0ZjVhNDc3NTk1NmI0ZDI5NGU2ZmEwNjQwM2E5NiIsInN1YiI6IjYyMGFjODJlY2FlMTdjMDA5MDljNTdmZiIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.KcxVpua42hmhJnDkRjRIOzhTZ4i3LmRz6FrDRtb0Zg0";
const API_KEY_V3 = "ec14f5a4775956b4d294e6fa06403a96";

//Always pass a callback to an async func to get the value outside (return statement KO)
const getPopular = async (callback) => {
  fetch(
    `https://api.themoviedb.org/3/movie/popular?api_key=${API_KEY_V3}&language=en-US&page=1`
  )
    .then((res) => res.json())
    .then((data) => {
      callback(data.results);
    })
    .catch((err) => callback(err));
};

async function fetchFilms() {
  const populars = await getPopular((films) => console.log(films));
}

fetchFilms();
