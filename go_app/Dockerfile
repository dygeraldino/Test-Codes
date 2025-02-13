FROM golang:1.23.6-bookworm

WORKDIR /go_app

COPY . /go_app

RUN go build -o go_app sc.go

CMD ["./go_app"]
