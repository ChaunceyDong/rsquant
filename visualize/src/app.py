from draw import Drawer
from flask import Flask
from markupsafe import Markup
from model import KlineEntry
from postgresql import setup_postgres
from service import Service

APP = Flask(__name__)
PG_CONN = setup_postgres()


def launch_server():
    print("Launch server in Debug mode.")
    APP.run(debug=True, host="0.0.0.0", port="5000")


@APP.route("/")
def hello_world():
    return "<h1>This is quant trader!</1>"


@APP.route("/kline/<symbol>")
def show_kline(symbol):
    klines = Service.get_kline(conn=PG_CONN, symbol=symbol)
    grid = Drawer.draw_kline(klines=klines, symbol=symbol)
    return Markup(grid.render_embed())


if __name__ == "__main__":
    launch_server()
