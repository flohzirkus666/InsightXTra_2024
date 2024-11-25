from fastapi import FastAPI, status
from pydantic import BaseModel

from create_share import create_cifs_share


class CifsShare(BaseModel):
    smb_name: str
    smb_size: int
    smb_permission: str


app = FastAPI()


# API endpoint
@app.post("/cifs_share")
async def create_share(share: CifsShare):
    """API Endpoint for creating a share"""
    # calling share create
    result = create_cifs_share(share.share_name, share.size, share.permission)
    if result:
        return status.HTTP_200_OK
    else:
        return status.HTTP_400_BAD_REQUEST
