"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const axios = require('axios');
const options = {
    method: 'GET',
    url: 'https://www.googleapis.com/youtube/v3/search',
    params: {
        key: 'AIzaSyAscOwuEycUYHEe-gt1o2bogRfd8RQsZ3A',
        q: 'the weekend',
        type: 'video',
        part: 'snippet',
    },
};
//NORMAL
axios
    .request(options)
    .then((response) => {
    console.log(response.data);
})
    .catch((error) => {
    console.error(error);
});
//ASYNC
const getData = () => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const response = yield axios.request(options);
        console.log(response);
    }
    catch (error) {
        console.log(error);
    }
});
