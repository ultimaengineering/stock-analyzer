FROM alpine:3.12.3
RUN apk update && apk upgrade
RUN mkdir /app
ADD /workspace/opt/app/shared/stock-analyzer /app/stock-analyzer
CMD /app/stock-analyzer
