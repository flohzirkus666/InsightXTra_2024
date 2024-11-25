from create_share import create_cifs_share
from flask import Flask, Response, request
from flask_cors import CORS

app = Flask(__name__)
CORS(app)


@app.route("/create_share", methods=["POST"])
def create_share():
    share_name = request.json.get("share_name")
    share_size = request.json.get("share_size")
    size_unit = request.json.get("size_prefix")
    permissions = request.json.get("smb_permission")

    size = f"{share_size}{size_unit}"

    result = create_cifs_share(share_name, size, permissions)
    if result:
        return Response(status=200)
    else:
        return Response(status=400)

if __name__ == "__main__":
    app.run(port=4000)