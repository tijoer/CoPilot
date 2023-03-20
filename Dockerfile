# nginx alpine image
FROM nginx:alpine

# Copy the html files to the nginx root
#COPY ./html /usr/share/nginx/html

# Expose port 80
EXPOSE 80
