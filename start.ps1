$linuxhost = "rhel1.demo.netapp.com"
$username = "root"
$password = "Netapp1!"

Write-Output "Setting language to German (Switzerland)";
Set-WinUserLanguageList -LanguageList "de-CH" -Force;

Write-Output "Download Git repository";
git clone https://github.com/flohzirkus666/InsightXTra_2024/

Write-Output "Zipping the contents of the downloaded Git repository";
Compress-Archive -Path "./InsightXTra_2024" -DestinationPath "./InsightXTra_2024.zip";

Write-Output "Copying the zip file to the remote Linux host";
scp ./InsightXTra_2024.zip $username@$linuxhost:/home/$username/;

Write-Output "Process completed successfully";

Write-Output "Connecting to remote Linux host, unzipping the file and running deploy_lod.sh";
ssh $username@$linuxhost 'unzip /home/root/InsightXTra_2024.zip -d /home/root/ && /home/root/InsightXTra_2024/deploy_lod.sh'

Write-Output "Deployment script executed successfully";