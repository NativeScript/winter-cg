//
//  NSCTextEncoder.swift
//  WinterCG
//
//  Created by Osei Fortune on 09/01/2024.
//

import Foundation

@objcMembers
@objc(NSCTextEncoder)
public class NSCTextEncoder: NSObject {
    private var encoder: OpaquePointer? = nil
    public override init() {
        super.init()
        encoder = wcg_core_text_encoder_create(nil)
    }
    
    public init(encoding: String){
        super.init()
        encoder = wcg_core_text_encoder_create(encoding.cString(using: .utf8))
    }
    
 
    public var encoding: String {
        let raw = wcg_core_text_encoder_get_encoding(encoder)
        if(raw == nil){
            return ""
        }
        let encoding = String(cString: raw!)
        let raw_mut  = UnsafeMutablePointer(mutating: raw)
        wcg_core_string_destroy(raw_mut)
        return encoding
    }
    
    public func encode(_ text: String) -> NSData {
        let txt = (text as NSString).utf8String
        let result = wcg_core_text_encoder_encode(encoder, txt)
        if(result == nil){
            return NSData()
        }
        let ptr = wcg_core_u8_buffer_get_bytes(result!)
        let len = wcg_core_u8_buffer_get_length(result!)
        
        // only making swift happy
        let ptr_mut = UnsafeMutableRawPointer(mutating: ptr)
        let bytes = NSData(bytesNoCopy: ptr_mut!, length: Int(len), deallocator: { ptr, count in
            wcg_core_u8_buffer_destroy(result!)
        })
        
        return bytes
    }
    
    
    
    deinit {
        wcg_core_text_encoder_destroy(encoder)
        encoder = nil
    }
}
