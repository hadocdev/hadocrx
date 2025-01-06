export namespace main {
	
	export class Profile {
	    name: string;
	    leftinfo: string;
	    rightinfo: string;
	    bottominfo: string;
	
	    static createFrom(source: any = {}) {
	        return new Profile(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.leftinfo = source["leftinfo"];
	        this.rightinfo = source["rightinfo"];
	        this.bottominfo = source["bottominfo"];
	    }
	}

}

