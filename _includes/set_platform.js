function detect_platform() {
  "use strict";
  var os = "unknown";

  if (os == "unknown") {
    if (navigator.platform == "Linux x86_64") {os = "x86_64-unknown-linux-gnu";}
    if (navigator.platform == "Linux i686") {os = "i686-unknown-linux-gnu";}
  }

  // I wish I knew by know, but I don't. Try harder.
  if (os == "unknown") {
    if (navigator.appVersion.indexOf("Win")!=-1) {os = "x86_64-pc-windows-gnu";}
    if (navigator.appVersion.indexOf("Mac")!=-1) {os = "x86_64-apple-darwin";}
    if (navigator.appVersion.indexOf("Linux")!=-1) {os = "x86_64-unknown-linux-gnu";}
  }

  return os;
}

(function () {
  "use strict";
  var platform = detect_platform();

  var rec_package_name = "1.4.0";
  var rec_version_type = "source";
  var rec_download_file = "rustc-1.4.0-src.tar.gz";

  if (platform == "x86_64-unknown-linux-gnu") {
    rec_version_type = "Linux binary";
    rec_download_file = "rust-" + rec_package_name + "-x86_64-unknown-linux-gnu.tar.gz";
  } else if (platform == "i686-unknown-linux-gnu") {
    rec_version_type = "Linux binary";
    rec_download_file = "rust-" + rec_package_name + "-i686-unknown-linux-gnu.tar.gz";
  } else if (platform == "x86_64-apple-darwin") {
    rec_version_type = "Mac installer";
    rec_download_file = "rust-" + rec_package_name + "-x86_64-apple-darwin.pkg";
  } else if (platform == "x86_64-pc-windows-gnu") {
    rec_version_type = "Windows installer";
    rec_download_file = "rust-" + rec_package_name + "-x86_64-pc-windows-gnu.msi";
  }

  var rec_package_desc = rec_package_name + " (<span>" + rec_version_type + "</span>)";
  var rec_vers_div = document.getElementById("install-version");
  rec_vers_div.innerHTML = rec_package_desc;

  var rec_dl_addy = "https://static.rust-lang.org/dist/" + rec_download_file;
  var rec_inst_link = document.getElementById("inst-link");
  rec_inst_link.setAttribute("href", rec_dl_addy);
}());
