from fastapi import FastAPI, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

from create_share import create_cifs_share


class CifsShare(BaseModel):
    smb_name: str
    smb_size: int
    smb_permission: str


app = FastAPI(port=4000)

origins = ["*"]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

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
