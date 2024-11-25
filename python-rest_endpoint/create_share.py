import os

from netapp_ontap import HostConnection
from netapp_ontap.error import NetAppRestError
from netapp_ontap.resources import CifsShare, Volume

# Getting connection from env
connection = HostConnection(
    os.environ["ONTAP_HOST"],
    os.environ["ONTAP_USERNAME"],
    os.environ["ONTAP_PASSWORD"],
    verify=False,
)


def create_cifs_share(
    share_name: str,
    size: str,
    permissions: str,
) -> bool:
    """A simple example for implementing a share creation."""
    # first step: creating a volume
    new_vol = Volume(
        name=f"{share_name}_vol",
        svm={"name": "cifs_svm"},
        size=size,
        nas={"path": f"/{share_name}".lower()},
        aggregates=[{"name": "cluster1_01_data"}],
    )
    # second step will be creating a share
    new_share = CifsShare()
    # we are setting here share acls according to our order
    new_share.acls = [
        {"permission": "full_control", "user_or_group": permissions, "type": "windows"}
    ]
    new_share.svm = {"name": "cifs_svm"}
    new_share.path = f"/{share_name}".lower()
    new_share.name = share_name
    with connection:
        try:
            # volume creation should be a blocking event
            new_vol.post(hydrate=True, poll=True)
            new_share.post(hydrate=True)
            # for showing off
            print(new_vol, new_share)
            # if creating was successul, return true
            return True
        except NetAppRestError:
            return False
