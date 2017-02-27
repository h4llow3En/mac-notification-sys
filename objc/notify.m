#import "notify.h"

/// getBundleIdentifier(app_name: &str) -> "de.hoodie.notify"
NSString *getBundleIdentifier(NSString *appName){
        NSString *findString = [NSString stringWithFormat:@"get id of application \"%@\"", appName];
        NSAppleScript *findScript = [[NSAppleScript alloc] initWithSource:findString];
        NSAppleEventDescriptor *resultDescriptor = [findScript executeAndReturnError:nil];
        return [resultDescriptor stringValue];
}

/// setApplication(new_bundle_identifier: &str) -> Result<()>
BOOL setApplication(NSString *newbundleIdentifier) {
        if(LSCopyApplicationURLsForBundleIdentifier((CFStringRef)newbundleIdentifier, NULL) != NULL) {
                fakeBundleIdentifier = newbundleIdentifier;
                return YES;
        }
        return NO;
}

/// scheduleNotification(title: &str, message: &str, sound: &str, f64) -> Result<()>
bool scheduleNotification(NSString *title, NSString *subtitle, NSString *message, NSString *sound, double deliveryDate) {
        @autoreleasepool {
                if (!installNSBundleHook()) {
                        return NO;
                }
                NSDate *scheduleTime = [NSDate dateWithTimeIntervalSince1970:deliveryDate];
                // NSDate *scheduleTime = [NSDate dateWithTimeIntervalSinceNow:deliveryDate];
                NSUserNotificationCenter *nc = [NSUserNotificationCenter defaultUserNotificationCenter];
                NotificationCenterDelegate *ncDelegate = [[NotificationCenterDelegate alloc] init];
                ncDelegate.keepRunning = YES;
                nc.delegate = ncDelegate;

                NSUserNotification *note = [[NSUserNotification alloc] init];
                note.title = title;
                note.subtitle = subtitle;
                note.informativeText = message;
                note.deliveryDate = scheduleTime;
                if (![sound isEqualToString:@"_mute"]) {
                        note.soundName = sound;
                }

                [nc scheduleNotification:note];
                return YES;
        }
}

bool sendNotification(NSString *title, NSString *subtitle, NSString *message, NSString *sound) {
        @autoreleasepool {
                if (!installNSBundleHook()) {
                        return NO;
                }

                NSUserNotificationCenter *nc = [NSUserNotificationCenter defaultUserNotificationCenter];
                NotificationCenterDelegate *ncDelegate = [[NotificationCenterDelegate alloc]init];
                ncDelegate.keepRunning = YES;
                nc.delegate = ncDelegate;

                NSUserNotification *note = [[NSUserNotification alloc] init];
                note.title = title;
                note.subtitle = subtitle;
                note.informativeText = message;
                if (![sound isEqualToString:@"_mute"]) {
                        note.soundName = sound;
                }
                [nc deliverNotification:note];

                while (ncDelegate.keepRunning) {
                        [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
                }
                return YES;
        }
}
