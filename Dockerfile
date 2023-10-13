FROM public.ecr.aws/nginx/nginx
COPY nginx.conf /etc/nginx/conf.d/default.conf
RUN mkdir -p /usr/share/nginx/html/site
COPY dist /usr/share/nginx/html/site/
