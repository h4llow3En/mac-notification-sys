#import "notify.h"

// getBundleIdentifier(app_name: &str) -> "com.apple.Terminal"
NSString* getBundleIdentifier(NSString* appName)
{
    NSString* findString = [NSString stringWithFormat:@"get id of application \"%@\"", appName];
    NSAppleScript* findScript = [[NSAppleScript alloc] initWithSource:findString];
    NSAppleEventDescriptor* resultDescriptor = [findScript executeAndReturnError:nil];
    return [resultDescriptor stringValue];
}

// setApplication(new_bundle_identifier: &str) -> Result<()>
BOOL setApplication(NSString* newbundleIdentifier)
{
    if (LSCopyApplicationURLsForBundleIdentifier((CFStringRef)newbundleIdentifier, NULL) != NULL)
    {
        fakeBundleIdentifier = newbundleIdentifier;
        return YES;
    }
    return NO;
}

NSImage* getImageFromURL(NSString* url)
{
    NSURL* imageURL = [NSURL URLWithString:url];
    if ([[imageURL scheme] length] == 0)
    {
        // Prefix 'file://' if no scheme
        imageURL = [NSURL fileURLWithPath:url];
    }
    return [[NSImage alloc] initWithContentsOfURL:imageURL];
}

void removeNotificationWithGroupID(NSString* groupID)
{
    NSUserNotificationCenter* center = [NSUserNotificationCenter defaultUserNotificationCenter];
    for (NSUserNotification* userNotification in center.deliveredNotifications)
    {
        if ([@"ALL" isEqualToString:groupID] || [userNotification.userInfo[@"groupID"] isEqualToString:groupID])
        {
            [center removeDeliveredNotification:userNotification];
            [center removeDeliveredNotification:userNotification];
        }
    }
}

// scheduleNotification(title: &str, subtitle: &str message: &str, sound: &str, f64) -> NotificationResult<()>
bool scheduleNotification(NSString* title, NSString* subtitle, NSString* message, NSString* sound, double deliveryDate)
{
    @autoreleasepool
    {
        if (!installNSBundleHook())
        {
            return NO;
        }
        NSDate* scheduleTime = [NSDate dateWithTimeIntervalSince1970:deliveryDate];
        NSUserNotificationCenter* nc = [NSUserNotificationCenter defaultUserNotificationCenter];
        NotificationCenterDelegate* ncDelegate = [[NotificationCenterDelegate alloc] init];
        ncDelegate.keepRunning = YES;
        nc.delegate = ncDelegate;

        NSUserNotification* userNotification = [[NSUserNotification alloc] init];
        userNotification.title = title;
        if (![subtitle isEqualToString:@""])
        {
            userNotification.subtitle = subtitle;
        }
        userNotification.informativeText = message;
        userNotification.deliveryDate = scheduleTime;
        if (![sound isEqualToString:@"_mute"])
        {
            userNotification.soundName = sound;
        }
        [nc scheduleNotification:userNotification];
        [NSThread sleepForTimeInterval:0.1f];
        return YES;
    }
}

// sendNotification(title: &str, subtitle: &str, message: &str, sound: &str) -> NotificationResult<()>
NSDictionary* sendNotification(NSString* title, NSString* subtitle, NSString* message, NSString* sound, NSDictionary* options)
{
    @autoreleasepool
    {
        if (!installNSBundleHook())
        {
            // TODO: Could potentially have different error messages
            return @{@"error" : @""};
        }

        // Remove earlier notification with the same group ID
        if (options[@"groupID"] && ![options[@"groupId"] isEqualToString:@""])
        {
            removeNotificationWithGroupID(options[@"groupID"]);
        }

        NSUserNotificationCenter* nc = [NSUserNotificationCenter defaultUserNotificationCenter];
        NotificationCenterDelegate* ncDelegate = [[NotificationCenterDelegate alloc] init];
        ncDelegate.keepRunning = YES;
        nc.delegate = ncDelegate;

        NSUserNotification* userNotification = [[NSUserNotification alloc] init];

        // Basic text
        userNotification.title = title;
        if (![subtitle isEqualToString:@""])
        {
            userNotification.subtitle = subtitle;
        }
        userNotification.informativeText = message;

        // Notification sound
        if (![sound isEqualToString:@"_mute"])
        {
            userNotification.soundName = sound;
        }

        // Main Actions Button (defaults to "Show")
        if (options[@"mainButtonLabel"] && ![options[@"mainButtonLabel"] isEqualToString:@""])
        {
            userNotification.actionButtonTitle = options[@"mainButtonLabel"];
            userNotification.hasActionButton = 1;
        }

        // Dropdown actions
        if (options[@"actions"] && ![options[@"actions"] isEqualToString:@""])
        {
            [userNotification setValue:@YES forKey:@"_showsButtons"];

            NSArray* myActions = [options[@"actions"] componentsSeparatedByString:@","];

            if (myActions.count > 1)
            {
                [userNotification setValue:@YES forKey:@"_alwaysShowAlternateActionMenu"];
                [userNotification setValue:myActions forKey:@"_alternateActionButtonTitles"];
            }
        }

        // Close/Other button (defaults to "Cancel")
        if (options[@"closeButtonLabel"] && ![options[@"closeButtonLabel"] isEqualToString:@""])
        {
            [userNotification setValue:@YES forKey:@"_showsButtons"];
            userNotification.otherButtonTitle = options[@"closeButtonLabel"];
        }

        // Reply to the notification with a text field
        if (options[@"response"])
        {
            userNotification.hasReplyButton = 1;
            userNotification.responsePlaceholder = options[@"responsePlaceholder"];
            NSLog(@"%@", options[@"responsePlaceholder"]);
        }

        // Change the icon of the app in the notification
        if (options[@"appIcon"])
        {
            NSImage* icon = getImageFromURL(options[@"appIcon"]);
            // replacement app icon
            [userNotification setValue:icon forKey:@"_identityImage"];
            [userNotification setValue:@(false) forKey:@"_identityImageHasBorder"];
        }
        // Change the additional content image
        if (options[@"contentImage"])
        {
            userNotification.contentImage = getImageFromURL(options[@"contentImage"]);
        }

        // [userNotification setValue:@(true) forKey:@"_clearable"];

        // TODO: Add more functionality like https://github.com/vjeantet/alerter/blob/master/alerter/AppDelegate.m

        [nc deliverNotification:userNotification];

        [NSThread sleepForTimeInterval:0.1f];

        // Loop/wait for a user action if needed
        while (ncDelegate.keepRunning)
        {
            [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
        }

        NSLog(@"%@", ncDelegate.actionData);

        return ncDelegate.actionData;
    }
}
