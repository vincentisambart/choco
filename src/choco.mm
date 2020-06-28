#import <Foundation/Foundation.h>
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

// We should not need that anymore once we can use the "C unwind" ABI.
#define ABORT_ON_EXCEPTION(expr) \
    @try { \
        expr \
    } \
    @catch (NSException *exception) { \
        abort_due_to_exception(exception); \
    }

// We want the C++ features, but not its name mangling.
extern "C" {

// Some explanation of the attributes used:
// - All Objective-C pointer return values must be marked NS_RETURNS_RETAINED.
//   That makes sure we always return an object with a +1 retain count.
// - All Objective-C pointer parameters must be marked __unsafe_unretained.
//   That makes sure that if the method called needs to keep a reference it will always increment the retain count before.
// 
// We end up with something similar to how Swift handles Objective-C pointers without having to think too much.

//-------------------------------------------------------------------
// NSObject

NSUInteger choco_core_NSObjectProtocol_instance_hash(id<NSObject> self_) {
    ABORT_ON_EXCEPTION(
        return self_.hash;
    )
}

BOOL choco_core_NSObjectProtocol_instance_isEqual(__unsafe_unretained id<NSObject> self_, __unsafe_unretained id object) {
    ABORT_ON_EXCEPTION(
        return [self_ isEqual:object];
    )
}

BOOL choco_core_NSObjectProtocol_instance_isKindOfClass(__unsafe_unretained id<NSObject> self_, Class klass) {
    ABORT_ON_EXCEPTION(
        return [self_ isKindOfClass:klass];
    )
}

Class choco_core_NSObject_class() {
    return [NSObject class];
}

NS_RETURNS_RETAINED NSObject *choco_core_NSObjectInterface_class_new(Class klass) {
    ABORT_ON_EXCEPTION(
        return [klass new];
    )
}

//-------------------------------------------------------------------
// NSString

Class choco_Foundation_NSString_class() {
    return [NSString class];
}

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

static_assert(std::is_same<NSStringEncoding, NSUInteger>::value, "expecting NSStringEncoding to be similar to NSUInteger");

NS_RETURNS_RETAINED NSString *choco_Foundation_NSStringInterface_class_newWithBytes_length_encoding(Class klass, const void *bytes, NSUInteger len, NSStringEncoding encoding) {
    ABORT_ON_EXCEPTION(
        return [[klass alloc] initWithBytes:bytes length:len encoding:encoding];
    )
}

//-------------------------------------------------------------------
// NSURL

Class choco_Foundation_NSURL_class() {
    return [NSURL class];
}

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

Class choco_Foundation_NSArray_class() {
    return [NSArray class];
}

NSUInteger choco_Foundation_NSArrayInterface_instance_count(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return self_.count;
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSArrayInterface_instance_firstObject(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return [self_ firstObject];
    )
}

NS_RETURNS_RETAINED id choco_Foundation_NSArrayInterface_instance_lastObject(__unsafe_unretained NSArray *self_) {
    ABORT_ON_EXCEPTION(
        return [self_ lastObject];
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

} // extern "C"
