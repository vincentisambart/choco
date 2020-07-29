#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#include <type_traits>
#include <cstdint>

#if !__has_feature(objc_arc)
#error This file must be compiled with ARC turned on (-fobjc-arc)
#endif

static_assert(std::is_same<BOOL, signed char>::value && sizeof(BOOL) == 1, "expecting BOOL to be a signed char");
static_assert(std::is_same<NSInteger, std::intptr_t>::value, "expecting NSInteger to be similar to isize");
static_assert(std::is_same<NSUInteger, std::size_t>::value, "expecting NSInteger to be similar to usize");

static void abort_due_to_exception(__unsafe_unretained NSException *exception) __attribute__((noreturn));

static void abort_due_to_exception(__unsafe_unretained NSException *exception) {
    NSLog(@"Unexpected exception: %@", exception.reason);
    abort();
}

// We want the C++ features, but not its name mangling.
extern "C" {

// We should not need that anymore once we can use the "C unwind" ABI.
#define ABORT_ON_EXCEPTION(expr) \
    @try { \
        expr \
    } \
    @catch (NSException *exception) { \
        abort_due_to_exception(exception); \
    }

#define CLASS_FUNCTION_DEFINITION(location, class_name) \
    Class choco_ ## location ## _ ## class_name ## _class(void) { \
        return [class_name class]; \
    }

// Some explanation of the attributes used:
// - All Objective-C pointer return values must be marked NS_RETURNS_RETAINED.
//   That makes sure we always return an object with a +1 retain count.
// - All Objective-C pointer parameters must be marked __unsafe_unretained.
//   That makes sure that if the method called needs to keep a reference it will always increment the retain count before.
// 
// We end up with something similar to how Swift handles Objective-C pointers without having to think too much.

//-------------------------------------------------------------------
// NSObject

NSUInteger choco_base_NSObjectProtocol_instance_hash(id<NSObject> self_) {
    ABORT_ON_EXCEPTION(
        return self_.hash;
    )
}

BOOL choco_base_NSObjectProtocol_instance_isEqual(__unsafe_unretained id<NSObject> self_, __unsafe_unretained id object) {
    ABORT_ON_EXCEPTION(
        return [self_ isEqual:object];
    )
}

BOOL choco_base_NSObjectProtocol_instance_isKindOfClass(__unsafe_unretained id<NSObject> self_, Class klass) {
    ABORT_ON_EXCEPTION(
        return [self_ isKindOfClass:klass];
    )
}

NS_RETURNS_RETAINED NSString *choco_base_NSObjectProtocol_instance_description(__unsafe_unretained id<NSObject> self_) {
    ABORT_ON_EXCEPTION(
        return self_.description;
    )
}

NS_RETURNS_RETAINED NSString *choco_base_NSObjectProtocol_instance_debugDescription(__unsafe_unretained id<NSObject> self_) {
    ABORT_ON_EXCEPTION(
        return self_.debugDescription;
    )
}

CLASS_FUNCTION_DEFINITION(base, NSObject)

NS_RETURNS_RETAINED NSObject *choco_base_NSObjectInterface_class_new(Class klass) {
    ABORT_ON_EXCEPTION(
        return [klass new];
    )
}

//-------------------------------------------------------------------
// NSCopying

NS_RETURNS_RETAINED id choco_Foundation_NSCopyingProtocol_instance_copy(__unsafe_unretained NSObject *self_) {
    ABORT_ON_EXCEPTION(
        return [self_ copy];
    )
}

//-------------------------------------------------------------------
// NSMutableCopying

NS_RETURNS_RETAINED id choco_Foundation_NSMutableCopyingProtocol_instance_mutableCopy(__unsafe_unretained NSObject *self_) {
    ABORT_ON_EXCEPTION(
        return [self_ mutableCopy];
    )
}

//-------------------------------------------------------------------
// NSFastEnumeration

static_assert(sizeof(NSFastEnumerationState) == 64, "expecting NSFastEnumerationState to be of size 64");

NSUInteger choco_Foundation_NSFastEnumerationProtocol_instance_countByEnumeratingWithState(__unsafe_unretained id<NSFastEnumeration> self_, NSFastEnumerationState *state, id  _Nullable __unsafe_unretained *buffer, NSUInteger len) {
    ABORT_ON_EXCEPTION(
        return [self_ countByEnumeratingWithState:state objects:buffer count:len];
    )
}

//-------------------------------------------------------------------
// NSString

static_assert(std::is_same<NSStringEncoding, NSUInteger>::value, "expecting NSStringEncoding to be similar to NSUInteger");

NS_RETURNS_RETAINED NSString *choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(Class klass, const void *bytes, NSUInteger len, NSStringEncoding encoding) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithBytes:bytes length:len encoding:encoding];
    )
}

CLASS_FUNCTION_DEFINITION(Foundation, NSString)

const char *choco_Foundation_NSStringInterface_instance_UTF8String(__unsafe_unretained NSString *self_) {
    ABORT_ON_EXCEPTION(
        return self_.UTF8String;
    )
}

static_assert(std::is_same<unichar, std::uint16_t>::value, "expecting unichar to be a std::uint16_t");
unichar choco_Foundation_NSStringInterface_instance_characterAtIndex(__unsafe_unretained NSString *self_, NSUInteger index) {
    ABORT_ON_EXCEPTION(
        return [self_ characterAtIndex:index];
    )
}

NSUInteger choco_Foundation_NSStringInterface_instance_length(__unsafe_unretained NSString *self_) {
    ABORT_ON_EXCEPTION(
        return self_.length;
    )
}

BOOL choco_Foundation_NSStringInterface_instance_isEqualToString(__unsafe_unretained NSString *self_, __unsafe_unretained NSString *object) {
    ABORT_ON_EXCEPTION(
        return [self_ isEqualToString:object];
    )
}

//-------------------------------------------------------------------
// NSMutableString

CLASS_FUNCTION_DEFINITION(Foundation, NSMutableString)

//-------------------------------------------------------------------
// NSURL

CLASS_FUNCTION_DEFINITION(Foundation, NSURL)

NS_RETURNS_RETAINED NSString *choco_Foundation_NSURLInterface_class_newWithString(Class klass, __unsafe_unretained NSString *urlString) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithString:urlString];
    )
}

NS_RETURNS_RETAINED NSString *choco_Foundation_NSURLInterface_class_fileURLWithPath(Class klass, __unsafe_unretained NSString *path) {
    ABORT_ON_EXCEPTION(
        // Note we do not call [klass fileURLWithPath:] to depend as less a possible on autorelease pools.
        return [[klass alloc] initFileURLWithPath:path];
    )
}

NS_RETURNS_RETAINED NSString *choco_Foundation_NSURLInterface_class_fileURLWithPath_isDirectory(Class klass, __unsafe_unretained NSString *path, BOOL isDirectory) {
    ABORT_ON_EXCEPTION(
        // Note we do not call [klass fileURLWithPath:] to depend as less a possible on autorelease pools.
        return [[klass alloc] initFileURLWithPath:path isDirectory:isDirectory];
    )
}

NS_RETURNS_RETAINED NSString *choco_Foundation_NSURLInterface_instance_absoluteString(__unsafe_unretained NSURL *self_) {
    ABORT_ON_EXCEPTION(
        return self_.absoluteString;
    )
}

//-------------------------------------------------------------------
// NSArray

CLASS_FUNCTION_DEFINITION(Foundation, NSArray)

NSUInteger choco_Foundation_NSArrayInterface_instance_count(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return self_.count;
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSArrayInterface_instance_firstObject(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return self_.firstObject;
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSArrayInterface_instance_lastObject(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return self_.lastObject;
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSArrayInterface_instance_objectAtIndex(__unsafe_unretained NSArray *self_, NSUInteger index) {
    ABORT_ON_EXCEPTION(
        return [self_ objectAtIndex:index];
    )
}

NS_RETURNS_RETAINED NSArray *choco_Foundation_NSArrayInterface_instance_arrayByAddingObject(__unsafe_unretained NSArray *self_, __unsafe_unretained id object) {
    ABORT_ON_EXCEPTION(
        return [self_ arrayByAddingObject:object];
    )
}

//-------------------------------------------------------------------
// NSMutableArray

CLASS_FUNCTION_DEFINITION(Foundation, NSMutableArray)

void choco_Foundation_NSMutableArrayInterface_instance_addObject(__unsafe_unretained NSMutableArray *self_, __unsafe_unretained id anObject) {
    ABORT_ON_EXCEPTION(
        return [self_ addObject:anObject];
    )
}

//-------------------------------------------------------------------
// NSDictionary

CLASS_FUNCTION_DEFINITION(Foundation, NSDictionary)

NSUInteger choco_Foundation_NSDictionaryInterface_instance_count(__unsafe_unretained NSDictionary *self_) {
    ABORT_ON_EXCEPTION(
        return self_.count;
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSDictionaryInterface_instance_objectForKey(__unsafe_unretained NSDictionary *self_, __unsafe_unretained id key) {
    ABORT_ON_EXCEPTION(
        return [self_ objectForKey:key];
    )
}

//-------------------------------------------------------------------
// NSMutableDictionary

CLASS_FUNCTION_DEFINITION(Foundation, NSMutableDictionary)

void choco_Foundation_NSMutableDictionaryInterface_instance_setObject_forKey(__unsafe_unretained NSMutableDictionary *self_, __unsafe_unretained id object, __unsafe_unretained id key) {
    ABORT_ON_EXCEPTION(
        return [self_ setObject:object forKey:key];
    )
}

void choco_Foundation_NSMutableDictionaryInterface_instance_removeObjectForKey(__unsafe_unretained NSMutableDictionary *self_, __unsafe_unretained id key) {
    ABORT_ON_EXCEPTION(
        return [self_ removeObjectForKey:key];
    )
}

void choco_Foundation_NSMutableDictionaryInterface_instance_removeAllObjects(__unsafe_unretained NSMutableDictionary *self_) {
    ABORT_ON_EXCEPTION(
        return [self_ removeAllObjects];
    )
}

//-------------------------------------------------------------------
// NSDate

CLASS_FUNCTION_DEFINITION(Foundation, NSDate)

static_assert(std::is_same<NSTimeInterval, double>::value, "expecting NSTimeInterval to be a double");

NSTimeInterval choco_Foundation_NSDateInterface_instance_timeIntervalSinceNow(__unsafe_unretained NSDate *self_) {
    ABORT_ON_EXCEPTION(
        return self_.timeIntervalSinceNow;
    )
}

NSTimeInterval choco_Foundation_NSDateInterface_instance_timeIntervalSinceReferenceDate(__unsafe_unretained NSDate *self_) {
    ABORT_ON_EXCEPTION(
        return self_.timeIntervalSinceReferenceDate;
    )
}

NSTimeInterval choco_Foundation_NSDateInterface_instance_timeIntervalSince1970(__unsafe_unretained NSDate *self_) {
    ABORT_ON_EXCEPTION(
        return self_.timeIntervalSince1970;
    )
}

NSTimeInterval choco_Foundation_NSDateInterface_instance_timeIntervalSinceDate(__unsafe_unretained NSDate *self_, __unsafe_unretained NSDate *anotherDate) {
    ABORT_ON_EXCEPTION(
        return [self_ timeIntervalSinceDate:anotherDate];
    )
}

//-------------------------------------------------------------------
// NSNumber

CLASS_FUNCTION_DEFINITION(Foundation, NSNumber)

NS_RETURNS_RETAINED NSNumber *choco_Foundation_NSNumberInterface_class_newWithBool(Class klass, BOOL value) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithBool:value];
    )
}

NS_RETURNS_RETAINED NSNumber *choco_Foundation_NSNumberInterface_class_newWithInteger(Class klass, NSInteger value) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithInteger:value];
    )
}

NS_RETURNS_RETAINED NSNumber *choco_Foundation_NSNumberInterface_class_newWithUnsignedInteger(Class klass, NSUInteger value) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithUnsignedInteger:value];
    )
}

BOOL choco_Foundation_NSNumberInterface_instance_boolValue(__unsafe_unretained NSNumber *self_) {
    ABORT_ON_EXCEPTION(
        return self_.boolValue;
    )
}

NSInteger choco_Foundation_NSNumberInterface_instance_integerValue(__unsafe_unretained NSNumber *self_) {
    ABORT_ON_EXCEPTION(
        return self_.integerValue;
    )
}

NSUInteger choco_Foundation_NSNumberInterface_instance_unsignedIntegerValue(__unsafe_unretained NSNumber *self_) {
    ABORT_ON_EXCEPTION(
        return self_.unsignedIntegerValue;
    )
}

//-------------------------------------------------------------------
// AVAsynchronousKeyValueLoading

AVKeyValueStatus choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_statusOfValueForKey_error(__unsafe_unretained id<AVAsynchronousKeyValueLoading> self_, __unsafe_unretained NSString *key, NSError * _Nullable __autoreleasing * _Nullable outError) {
    ABORT_ON_EXCEPTION(
        return [self_ statusOfValueForKey:key error:outError];
    )
}

void choco_AVFoundation_AVAsynchronousKeyValueLoadingProtocol_instance_loadValuesAsynchronouslyForKeys_completionHandler(__unsafe_unretained id<AVAsynchronousKeyValueLoading> self_, __unsafe_unretained NSArray<NSString *> *keys, void (^ __unsafe_unretained handler)(void)) {
    ABORT_ON_EXCEPTION(
        return [self_ loadValuesAsynchronouslyForKeys:keys completionHandler:handler];
    )
}

//-------------------------------------------------------------------
// AVAsset

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAsset)

NS_RETURNS_RETAINED NSArray<AVAssetTrack *> *choco_AVFoundation_AVAssetInterface_instance_tracks(__unsafe_unretained AVAsset *self_) {
    ABORT_ON_EXCEPTION(
        return self_.tracks;
    )
}

BOOL choco_AVFoundation_AVAssetInterface_instance_playable(__unsafe_unretained AVAsset *self_) {
    ABORT_ON_EXCEPTION(
        return self_.playable;
    )
}

//-------------------------------------------------------------------
// AVURLAsset

CLASS_FUNCTION_DEFINITION(AVFoundation, AVURLAsset)

NS_RETURNS_RETAINED AVURLAsset *choco_AVFoundation_AVURLAssetInterface_class_newWithURL_options(Class klass, __unsafe_unretained NSURL *url, __unsafe_unretained NSDictionary<NSString *,id> *options) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithURL:url options:options];
    )
}

//-------------------------------------------------------------------
// AVAssetTrack

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAssetTrack)

NS_RETURNS_RETAINED NSString *choco_AVFoundation_AVAssetTrackInterface_instance_mediaType(__unsafe_unretained AVAssetTrack *self_) {
    ABORT_ON_EXCEPTION(
        return self_.mediaType;
    )
}

NS_RETURNS_RETAINED NSArray *choco_AVFoundation_AVAssetTrackInterface_instance_formatDescriptions(__unsafe_unretained AVAssetTrack *self_) {
    ABORT_ON_EXCEPTION(
        return self_.formatDescriptions;
    )
}

//-------------------------------------------------------------------
// AVAssetReader

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAssetReader)

NS_RETURNS_RETAINED AVURLAsset *choco_AVFoundation_AVAssetReaderInterface_class_newWithAsset_error(Class klass, __unsafe_unretained AVAsset *asset, NSError * _Nullable __autoreleasing * _Nullable outError) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithAsset:asset error:outError];
    )
}

//-------------------------------------------------------------------
// AVAssetReaderTrackOutput

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAssetReaderTrackOutput)

//-------------------------------------------------------------------
// AVAssetReaderOutput

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAssetReaderOutput)

//-------------------------------------------------------------------
// AVAssetReaderSampleReferenceOutput

CLASS_FUNCTION_DEFINITION(AVFoundation, AVAssetReaderSampleReferenceOutput)

} // extern "C"
