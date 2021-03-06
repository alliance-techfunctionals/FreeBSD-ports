--- vendor/rustc-ap-rustc_target/spec/armv7_unknown_freebsd.rs.orig	2019-02-15 13:41:07 UTC
+++ vendor/rustc-ap-rustc_target/spec/armv7_unknown_freebsd.rs
@@ -0,0 +1,24 @@
+use spec::{LinkerFlavor, Target, TargetOptions, TargetResult};
+
+pub fn target() -> TargetResult {
+    let base = super::freebsd_base::opts();
+    Ok(Target {
+        llvm_target: "armv7-unknown-freebsd-gnueabihf".to_string(),
+        target_endian: "little".to_string(),
+        target_pointer_width: "32".to_string(),
+        target_c_int_width: "32".to_string(),
+        data_layout: "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
+        arch: "arm".to_string(),
+        target_os: "freebsd".to_string(),
+        target_env: "gnueabihf".to_string(),
+        target_vendor: "unknown".to_string(),
+        linker_flavor: LinkerFlavor::Gcc,
+
+        options: TargetOptions {
+            features: "+v7,+vfp3,+d16,+thumb2,-neon".to_string(),
+            max_atomic_width: Some(64),
+            abi_blacklist: super::arm_base::abi_blacklist(),
+            .. base
+        }
+    })
+}
