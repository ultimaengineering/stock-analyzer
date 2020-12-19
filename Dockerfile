FROM alpine:3.12.3
RUN apk update && apk upgrade
RUN mkdir /app
RUN ls /workspace/opt/app/shared/
ADD /workspace/stock-analyzer /app/stock-analyzer
CMD /app/stock-analyzer
