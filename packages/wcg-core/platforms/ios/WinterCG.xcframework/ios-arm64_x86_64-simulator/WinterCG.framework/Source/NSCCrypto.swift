//
//  NSCCrypto.swift
//  WinterCG
//
//  Created by Osei Fortune on 09/01/2024.
//

import Foundation


enum CustomError: Error {
    case runtimeError(String)
}

@objcMembers
@objc(NSCCrypto)
public class NSCCrypto: NSObject {
    
    
    public static func getRandomValuesBytes(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: UInt8.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesUShort(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 2
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: UInt16.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesUInt(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 4
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: UInt32.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesULong(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 8
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: UInt64.self).advanced(by: offset), UInt(size))
    }
    
    
    

    
    public static func getRandomValues(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: Int8.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesShort(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 2
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: Int16.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesInt(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 4
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: Int32.self).advanced(by: offset), UInt(size))
    }
    
    
    public static func getRandomValuesLong(_ value: UnsafeMutableRawPointer,_ size: Int, _ offset: Int) throws {
        let size = size * 8
        if (size > 65536){
            throw CustomError.runtimeError("The ArrayBufferView's byte length \(size) exceeds the number of bytes of entropy available via this API (65536)")
        }
    
        
        wcg_core_crypto_get_random_values(value.assumingMemoryBound(to: Int64.self).advanced(by: offset), UInt(size))
    }
    
    
    
    public static func randomUUID() -> String {
        let raw =  wcg_core_crypto_random_uuid()
        if(raw == nil){
            return ""
        }
        let encoding = String(cString: raw!)
        let raw_mut  = UnsafeMutablePointer(mutating: raw)
        wcg_core_string_destroy(raw_mut)
        return encoding
    }
}
