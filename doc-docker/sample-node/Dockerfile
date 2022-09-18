FROM node:alpine
# cd in /usr/app if the directory does not exist created it
WORKDIR /usr/app
# Needed for npm install
COPY ./package.json ./
# run npm install
RUN npm install
# copy the rest of the files (if something change to the source code npm is not relaunched)
# Very important to adjust the process for each build if the image is rebuild in production
# This prevent a real waste of time and resources
COPY ./ ./
CMD [ "npm", "start" ]
