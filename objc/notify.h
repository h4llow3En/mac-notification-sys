#import <Cocoa/Cocoa.h>
#import <CoreServices/CoreServices.h>
#import <Foundation/Foundation.h>
#import <objc/runtime.h>

NSString* fakeBundleIdentifier = nil;

NSString* getBundleIdentifier(NSString* appName);
BOOL setApplication(NSString* newbundleIdentifier);
void sendNotification(NSString* title, NSString* subtitle, NSString* message, NSDictionary* options, const char* notificationId, BOOL shouldWait);
void ensureDelegateInitiated(void);

// Rust callbacks — implemented in lib.rs, called from ObjC delegate
extern void rust_notification_activated(const char* uuid, const char* activationType, const char* actionValue, const char* actionValueIndex);
extern void rust_notification_dismissed(const char* uuid, const char* buttonTitle);
extern void rust_notification_auto_dismissed(const char* uuid);
extern BOOL rust_notification_is_done(const char* uuid);
extern void rust_wait_for_notification(const char* uuid);
// Delivery-confirmation callbacks (fire-and-forget path)
extern void rust_notification_delivered(const char* uuid);
extern BOOL rust_notification_is_delivered(const char* uuid);
extern void rust_wait_for_delivery(const char* uuid);

@implementation NSBundle (swizzle)
- (NSString*)__bundleIdentifier {
    if (self == [NSBundle mainBundle]) {
        return fakeBundleIdentifier ? fakeBundleIdentifier : @"com.apple.Terminal";
    } else {
        return [self __bundleIdentifier];
    }
}
@end

BOOL installNSBundleHook(void) {
    Class class = objc_getClass("NSBundle");
    if (class) {
        method_exchangeImplementations(class_getInstanceMethod(class, @selector(bundleIdentifier)),
                                       class_getInstanceMethod(class, @selector(__bundleIdentifier)));
        return YES;
    }
    return NO;
}

@interface NotificationCenterDelegate: NSObject <NSUserNotificationCenterDelegate>
+ (instancetype)sharedDelegate;
@end
