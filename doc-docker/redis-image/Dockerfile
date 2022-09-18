# Use an existing IMAGE
FROM  alpine 

#Install dependency (apk is the package manager for alpine)
RUN apk add --update redis
RUN apk add --update gcc

RUN mkdir osef && touch /osef/Carl.txt

#Tell the image what to do when it start as a container
CMD ["redis-server"]

# LAYERS

# At each step a temporary container is created and the command is executed inside of it
# then a snapshot of the file system  of the temporary container is used to build a 
# new image and the temporary container is deleted.
# The last image is our final image.

# ORDER MATTER

# Docker server when rebuild image uses cache it can remembeer our instruction to rebuild
# our image blazingly fast fot that it's important to add new instruction always at the bottom
# if Docker server find a new instruction the cache can not be used for the rest.
