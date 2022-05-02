use crate::common::{ClipboardProvider, Result};
use jni::objects::JString;
use std::ffi::CStr;

pub struct AndroidClipboardContext;

impl ClipboardProvider for AndroidClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let env = vm.attach_current_thread()?;
        let class_ctx = env.find_class("android/content/Context")?;
        let cb = env.get_static_field(class_ctx, "CLIPBOARD_SERVICE", "Ljava/lang/String;")?;
        let cb_manager = env
            .call_method(
                ctx.context() as jni::sys::jobject,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[cb],
            )?
            .l()
            .unwrap();

        let clip_data = env
            .call_method(
                cb_manager,
                "getPrimaryClip",
                "()Landroid/content/ClipData;",
                &[],
            )?
            .l()
            .unwrap();

        //return Ok(format!("{:?}", clip_data));

        let item = env
            .call_method(
                clip_data,
                "getItemAt",
                "(I)Landroid/content/ClipData$Item;",
                &[0i32.into()],
            )?
            .l()
            .unwrap();

        let char_seq = env
            .call_method(item, "getText", "()Ljava/lang/CharSequence;", &[])?
            .l()?;

        let string = env
            .call_method(char_seq, "toString", "()Ljava/lang/String;", &[])?
            .l()
            .unwrap();

        let jstring = JString::from(string.into_inner());

        let ptr = env.get_string_utf_chars(jstring)?;
        let s;
        unsafe {
            s = CStr::from_ptr(ptr).to_owned().into_string()?;
        }
        env.release_string_utf_chars(jstring, ptr)?;
        Ok(s)
    }

    fn set_contents(&mut self, text: String) -> Result<()> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let env = vm.attach_current_thread()?;
        let class_ctx = env.find_class("android/content/Context")?;
        let cb = env.get_static_field(class_ctx, "CLIPBOARD_SERVICE", "Ljava/lang/String;")?;
        let cb_manager = env
            .call_method(
                ctx.context() as jni::sys::jobject,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[cb],
            )?
            .l()
            .unwrap();

        let class_clip_data = env.find_class("android/content/ClipData")?;

        let clip_data = env.call_static_method(
            class_clip_data,
            "newPlainText",
            "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
            &[
                env.new_string("text").unwrap().into(),
                env.new_string(text).unwrap().into(),
            ],
        )?;

        env.call_method(
            cb_manager,
            "setPrimaryClip",
            "(Landroid/content/ClipData;)V",
            &[clip_data],
        )?
        .v()?;

        Ok(())
    }
}

impl AndroidClipboardContext{
    #[inline]
    pub fn new() -> Result<Self>{
        Ok(AndroidClipboardContext)
    }
}