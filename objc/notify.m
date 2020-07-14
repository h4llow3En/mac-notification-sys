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
    NSURL *imageURL = [NSURL URLWithString:url];
    if([[imageURL scheme] length] == 0){
        // Prefix 'file://' if no scheme
        imageURL = [NSURL fileURLWithPath:url];
    }
    return [[NSImage alloc] initWithContentsOfURL:imageURL];
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

        NSUserNotification* note = [[NSUserNotification alloc] init];
        note.title = title;
        if (![subtitle isEqualToString:@""])
        {
            note.subtitle = subtitle;
        }
        note.informativeText = message;
        note.deliveryDate = scheduleTime;
        if (![sound isEqualToString:@"_mute"])
        {
            note.soundName = sound;
        }
        [nc scheduleNotification:note];
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
            return @{ @"error": @"" };
        }

        NSUserNotificationCenter* nc = [NSUserNotificationCenter defaultUserNotificationCenter];
        NotificationCenterDelegate* ncDelegate = [[NotificationCenterDelegate alloc] init];
        ncDelegate.keepRunning = YES;
        nc.delegate = ncDelegate;

        NSUserNotification* note = [[NSUserNotification alloc] init];

        // Basic text
        note.title = title;
        if (![subtitle isEqualToString:@""])
        {
            note.subtitle = subtitle;
        }
        note.informativeText = message;

        // Notification sound
        if (![sound isEqualToString:@"_mute"])
        {
            note.soundName = sound;
        }

        // Main action button
        if (options[@"actionButtonTitle"] && ![options[@"actionButtonTitle"] isEqualToString:@""]) {
            [note setValue:@YES forKey:@"_showsButtons"];
            note.hasActionButton = YES;
            note.actionButtonTitle = options[@"actionButtonTitle"];
        }
        else {
            note.hasActionButton = NO;
        }

        // Other button (defaults to "Cancel")
        if (options[@"otherButtonTitle"] && ![options[@"otherButtonTitle"] isEqualToString:@""]) {
            [note setValue:@YES forKey:@"_showsButtons"];
            note.otherButtonTitle = options[@"otherButtonTitle"];
        }
        
        // Reply to the notification with a text field
        if (options[@"response"])
        {
            note.hasReplyButton = 1;
            note.responsePlaceholder = options[@"responsePlaceholder"];
            NSLog(@"%@", options[@"responsePlaceholder"]);
        }

        // Change the icon of the app in the notification
        if(options[@"appIcon"]){
            NSImage* icon = getImageFromURL(options[@"appIcon"]);
            // replacement app icon
            [note setValue:icon forKey:@"_identityImage"];
            [note setValue:@(false) forKey:@"_identityImageHasBorder"];
        }

        // [note setValue:@(true) forKey:@"_clearable"];

        // TODO: Add more functionality like https://github.com/vjeantet/alerter/blob/master/alerter/AppDelegate.m

        [nc deliverNotification:note];

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
