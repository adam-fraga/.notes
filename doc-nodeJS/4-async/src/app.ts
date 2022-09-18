const axios = require('axios')

const options = {
  method: 'GET',
  url: 'https://www.googleapis.com/youtube/v3/search',
  params: {
    key: 'AIzaSyAscOwuEycUYHEe-gt1o2bogRfd8RQsZ3A',
    q: 'the weekend',
    type: 'video',
    part: 'snippet',
  },
}

//NORMAL
axios
  .request(options)
  .then((response: any) => {
    console.log(response.data)
  })
  .catch((error: any) => {
    console.error(error)
  })

//ASYNC
const getData = async () => {
  try {
    const response = await axios.request(options)
    console.log(response)
  } catch (error) {
    console.log(error)
  }
}
