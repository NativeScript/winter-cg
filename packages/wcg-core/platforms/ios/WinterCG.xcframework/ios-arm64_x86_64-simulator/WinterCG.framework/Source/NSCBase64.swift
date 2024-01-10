//
//  NSCBase64.swift
//  WinterCG
//
//  Created by Osei Fortune on 09/01/2024.
//

import Foundation

@objcMembers
@objc(NSCBase64)
public class NSCBase64: NSObject {
    public static func atob(_ value: String) -> String {
        let decoded = wcg_core_atob(value.cString(using: .utf8))
        if(decoded == nil){
            return ""
        }
       
        let ret = String(cString: decoded!)
        
        let decoded_mut  = UnsafeMutablePointer(mutating: decoded)
        wcg_core_string_destroy(decoded_mut)
        
        return ret
    }
    
    
    public static func btoa(_ value: String) -> String {
        let encoded = wcg_core_atob(value.cString(using: .utf8))
        if(encoded == nil){
            return ""
        }
       
        let ret = String(cString: encoded!)
        
        let encoded_mut  = UnsafeMutablePointer(mutating: encoded)
        wcg_core_string_destroy(encoded_mut)
        
        return ret
    }
}
