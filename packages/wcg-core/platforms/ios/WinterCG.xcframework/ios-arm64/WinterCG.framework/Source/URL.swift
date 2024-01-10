//
//  URL.swift
//  WinterCG
//
//  Created by Osei Fortune on 09/01/2024.
//

import Foundation

@objcMembers
@objc(NSCWCGURL)
public class NSCWCGURL: NSObject {
    
    private var url: OpaquePointer
    
    public init(value: String, base: String? = nil){
        if(base == nil){
            url = wcg_core_url_create(value.cString(using: .utf8), nil)
        }else {
            url =  wcg_core_url_create(value.cString(using: .utf8), base!.cString(using: .utf8))
        }

    }
    
    public var uhash: String {
        get {
            let hash_value = wcg_core_url_hash(url)
            if(hash_value == nil){
                return ""
            }
            let ret = String(cString: hash_value!)
            
            let hash_value_mut  = UnsafeMutablePointer(mutating: hash_value)
            wcg_core_string_destroy(hash_value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_hash(url, newValue.cString(using: .utf8))
        }
    }
    
    public var host: String {
        get {
            let value = wcg_core_url_host(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_host(url, newValue.cString(using: .utf8))
        }
    }
    
    
    public var hostname: String {
        get {
            let value = wcg_core_url_host_name(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_host_name(url, newValue.cString(using: .utf8))
        }
    }
        
 
    public var href: String {
        get {
            let value = wcg_core_url_href(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_href(url, newValue.cString(using: .utf8))
        }
    }
    
    public var origin: String {
        get {
            let value = wcg_core_url_origin(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
    }
    
    public var password: String {
        get {
            let value = wcg_core_url_password(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_password(url, newValue.cString(using: .utf8))
        }
    }
    
    
    public var pathname: String {
        get {
            let value = wcg_core_url_pathname(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_pathname(url, newValue.cString(using: .utf8))
        }
    }
    

    
    public var port: String {
        get {
            let value = wcg_core_url_port(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_port(url, newValue.cString(using: .utf8))
        }
    }


    
    // name changed to prevent clash
    public var uprotocol: String {
        get {
            let value = wcg_core_url_protocol(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_protocol(url, newValue.cString(using: .utf8))
        }
    }




    public var search: String {
        get {
            let value = wcg_core_url_search(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_search(url, newValue.cString(using: .utf8))
        }
    }
    
    public var username: String {
        get {
            let value = wcg_core_url_username(url)
            if(value == nil){
                return ""
            }
            let ret = String(cString: value!)
            
            let value_mut  = UnsafeMutablePointer(mutating: value)
            wcg_core_string_destroy(value_mut)
            
            return ret
        }
        set {
            wcg_core_url_set_username(url, newValue.cString(using: .utf8))
        }
    }
    
    
  
   
    
    public func toString() -> String {
        let value = wcg_core_url_to_string(url)
        if(value == nil){
            return ""
        }
        
        let ret = String(cString: value!)
        
        let value_mut = UnsafeMutablePointer(mutating: value)
        wcg_core_string_destroy(value_mut)
        
        return ret
        
    }
    
    deinit {
        wcg_core_url_destroy(url)
    }
    
    public static func canParse(_ value: String, _ base: String?) -> Bool {
        if(base == nil){
           return wcg_core_url_can_parse(value.cString(using: .utf8), nil)
        }else {
          return  wcg_core_url_can_parse(value.cString(using: .utf8), base!.cString(using: .utf8))
        }
       
    }
}
