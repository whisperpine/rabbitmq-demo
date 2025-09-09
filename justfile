# open the RabbitMQ management web page in the default browser
web:
  xdg-open "http://localhost:15672"

# cargo nextest the given test case(s) and output logs
t CASE:
  cargo nextest run {{CASE}} --no-capture
