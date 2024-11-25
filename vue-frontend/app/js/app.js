const { createApp, ref, reactive } = Vue;

createApp({
    setup() {
        // Set initial state
        let isActive = ref(true);
        let modalClass = reactive({
            'visually-hidden': true,
            'alert-success': true,
            'alert-danger': false,
            'alert': true,
            'alert-dismissible': true,
        });
        let alertMsg = '';

        return { isActive, modalClass, alertMsg };
    },
    methods: {
        async createCifs() {
            // Disable button
            this.isActive = false;
            // Send response to backend
            try {
                const smbName = this.$refs.smb_name.value;
                const smbSize = this.$refs.smb_size.value.slice(0, -2);
                const smbSizePrefix = this.$refs.smb_size.value.slice(-2);
                const smbPermission = this.$refs.smb_permission.value;

                const response = await fetch('http://192.168.10.2/cifs_share', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Access-Control-Allow-Origin': '*', // CORS
                    },
                    body: JSON.stringify({
                        smb_name: smbName,
                        smb_size: Number(smbSize),
                        smb_size_prefix: smbSizePrefix,
                        smb_permission: smbPermission
                    })
                });

                if (response.ok) {
                    const data = await response.json();
                    console.log('Success:', data);
                    this.successAlert();
                } else {
                    this.failureAlert();
                    console.error('Error:', response.statusText);
                }
            } catch (error) {
                console.error('Error:', error);
            }
            // Enable button
            this.isActive = true;
        },
        async createNfs() {
            // Disable button
            this.isActive = false;
            // Send response to backend
            try {
                const nfsName = this.$refs.nfs_path.value;
                const nfsSize = this.$refs.nfs_size.value.slice(0, -2);
                const sizePrefix = this.$refs.nfs_size.value.slice(-2);

                const response = await fetch('http://192.168.10.3/nfs_share', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Access-Control-Allow-Origin': '*', // CORS
                    },
                    body: JSON.stringify({
                        volume_name: nfsName,
                        size: Number(nfsSize),
                        prefix: sizePrefix,
                    })
                });

                if (response.ok) {
                    const data = await response.json();
                    console.log('Success:', data);
                    this.successAlert();
                } else {
                    console.error('Error:', response.statusText);
                    this.failureAlert();
                }
            } catch (error) {
                console.error('Error:', error);
            }
            // Enable button
            this.isActive = true;
        },
        successAlert() {
            // CSS mofiications for success alert
            this.alertMsg = '<strong>Success!</strong> Your share has been created.';
            this.modalClass['visually-hidden'] = false;
            this.modalClass['alert-success'] = true;
        },
        failureAlert() {
            // CSS modifications for failure alert
            this.alertMsg = '<strong>Failure!</strong> Your share could not be created.';
            this.modalClass['visually-hidden'] = false;
            this.modalClass['alert-danger'] = true;
        },
        closeAlert() {
            // Close alert
            this.modalClass['visually-hidden'] = true;
        }
    },
    template: `
    <div class="container">
    <div class="row mt-2">
        <div :class="modalClass" role="alert">
            <span v-html="alertMsg"></span>
            <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close" @click="closeAlert"></button>
        </div>
    </div>
        <div class="row mt-3">
            <h3>Order a CIFS Share</h3>
        </div>
        <form id="smb_order_form">
            <div class="row">
                <div class="col-3">
                    <label for="smb_name" class="form-label">Share name</label>
                    <input type="text" class="form-control" ref="smb_name" name="smb_name">
                </div>
                <div class="col-3">
                    <label for="smb_size" class="form-label">Share size</label>
                    <input type="text" class="form-control" ref="smb_size" name="smb_size">
                </div>
            </div>
            <div class="row mt-3">
                <div class="col-6">
                    <select class="form-select" aria-label="permission selection" ref="smb_permission" name="smb_permission">
                        <option selected>Choose permission level</option>
                        <option value="Everyone">Everyone</option>
                        <option value="Authenticated Users">Authenticated Users</option>
                    </select>
                </div>
            </div>
            <div class="row mt-3">
                <div class="col-3">
                    <button class="btn btn-primary" type="submit" @click="createCifs" :disabled="!isActive">Create Share</button>
                </div>
            </div>
        </form>
    </div>
        <div class="container">
        <div class="row mt-3">
            <h3>Order a NFS Export</h3>
        </div>
        <form id="nfs_order_form">
            <div class="row">
                <div class="col-3">
                    <label for="nfs_path" class="form-label">Mount Path</label>
                    <input type="text" class="form-control" ref="nfs_path" name="nfs_path">
                </div>
                <div class="col-3">
                    <label for="nfs_size" class="form-label">Share size</label>
                    <input type="text" class="form-control" ref="nfs_size" name="nfs_size">
                </div>
            </div>
            <div class="row mt-3">
                <div class="col-3">
                    <button class="btn btn-primary" type="submit" @click="createNfs" :disabled="!isActive">Create Share</button>
                </div>
            </div>
        </form>
    </div>
    `
}).mount('#app');
